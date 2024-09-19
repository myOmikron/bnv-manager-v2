use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use tracing::{debug, instrument, warn};
use tracing::log::error;
use uuid::Uuid;

use conf_updater_common::ApiFailure;

use crate::config::{Config, MiscConfig};
use crate::utils::{ProgramError, try_from_utf8};
use crate::utils::web_space::get_webroot;

/// Configure a new or existing website with nginx
#[instrument(level = "trace")]
pub(crate) fn write_nginx_conf(
    website: &Uuid,
    user_uuid: &Uuid,
    hosted_domains: &Vec<String>,
    forwarded_domains: &Vec<String>,
    all_domains: &Vec<String>,
    conf: &Config,
) -> Result<(), ApiFailure> {
    let nginx_conf_content = create_nginx_config_content(website, &user_uuid, hosted_domains, forwarded_domains, all_domains, conf)?;
    debug!("Full new nginx config: {}", nginx_conf_content);

    // Write the new configuration file, then change the permissions correctly
    let conf_file = PathBuf::from(&conf.misc.nginx_config_dir).join(website.as_hyphenated().to_string());
    if conf_file.exists() {
        fs::remove_file(&conf_file)?;
    };
    {
        let mut file = File::create_new(&conf_file)?;
        file.write(nginx_conf_content.as_bytes())?;
        file.sync_all()?;
    }
    {
        let output = Command::new("chgrp")
            .arg(&conf.misc.nginx_group)
            .arg(&conf_file)
            .output()?;
        if !output.status.success() {
            warn!("{}", try_from_utf8("chgrp-nginx-conf".to_string(), &output.stderr)?);
        }
    }
    {
        let output = Command::new("chmod")
            .arg("0640")
            .arg(&conf_file)
            .output()?;
        if !output.status.success() {
            warn!("{}", try_from_utf8("chmod-nginx-conf".to_string(), &output.stderr)?);
        }
    }

    Ok(())
}

/// Check that the `nginx` executable is found by `which`
pub(crate) fn check_available() -> bool {
    let result = Command::new("which").arg("nginx").output();
    match result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Reload the nginx web server, returning the utility's response on error
pub(crate) fn reload_server() -> Result<(), ProgramError> {
    let output = Command::new("nginx").arg("-s").arg("reload").output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = try_from_utf8("nginx-reload".to_string(), &*output.stderr)?;
        Err(ProgramError::Failure("nginx-reload".to_string(), stderr))
    }
}

/// Verify the current nginx web server configuration, returning errors on failure
pub(crate) fn verify_config() -> Result<(), ProgramError> {
    let output = Command::new("nginx").arg("-t").output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = try_from_utf8("nginx-verify-conf".to_string(), &*output.stderr)?;
        Err(ProgramError::Failure("nginx-verify-conf".to_string(), stderr))
    }
}

/// Remove an existing nginx configuration for a particular website (no error if the website doesn't exist)
#[instrument(level = "trace", skip(conf))]
pub(crate) fn drop_config(website: &Uuid, conf: &MiscConfig) -> Result<(), ProgramError> {
    let conf_file = PathBuf::from(&conf.nginx_config_dir).join(website.as_hyphenated().to_string());
    if conf_file.exists() {
        fs::remove_file(&conf_file)?;
    };
    Ok(())
}

/// Produce the content for a nginx configuration file
#[instrument(level = "trace", skip(conf))]
fn create_nginx_config_content(
    website: &Uuid,
    user_uuid: &Uuid,
    hosted_domains: &Vec<String>,
    forwarded_domains: &Vec<String>,
    all_domains: &Vec<String>,
    conf: &Config,
) -> Result<String, ApiFailure> {
    let certbot_tls_dir = &conf.certbot.cert_dir;
    let website_uuid = website.as_hyphenated().to_string();
    let target_domain = hosted_domains.get(0).ok_or(ApiFailure::BadRequest("Missing at least one target domain".to_string()))?;
    let root_dir_path = get_webroot(user_uuid, website, &conf.misc);
    let root_dir = match root_dir_path.to_str() {
        Some(v) => Ok(v),
        None => {
            error!("Formatting webroot as string did not work for {} / {}", user_uuid.as_hyphenated(), website.as_hyphenated());
            Err(ApiFailure::InternalServerError)
        }
    }?;

    // Since Rust requires string literals in the format! macro,
    // these blocks of text need to be inlined into this function
    let forwarding_extension = if forwarded_domains.len() == 0 { "".to_string() } else {
        format!("\n\nserver {{
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name {};

    root {root_dir};
    index index.html index.htm;

    location / {{
        return 302 https://{target_domain}/$request_uri;
    }}

    ssl_certificate {certbot_tls_dir}/{website_uuid}/fullchain.pem;
    ssl_certificate_key {certbot_tls_dir}/{website_uuid}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}}", forwarded_domains.join(" "))
    };

    let https_extension = if conf.certbot.test_certs { "".to_string() } else {
        format!("\n\nserver {{
   listen 80;
   listen [::]:80;
   server_name {};
   return 301 https://$host$request_uri;
}}", all_domains.join(" "))
    };

    let full_config = format!("# This file is managed by the web conf updater

server {{
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name {};

    root {root_dir};
    index index.html index.htm;

    location / {{
        try_files $uri $uri/ =404;
    }}

    ssl_certificate {certbot_tls_dir}/{website_uuid}/fullchain.pem;
    ssl_certificate_key {certbot_tls_dir}/{website_uuid}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}}{forwarding_extension}{https_extension}", hosted_domains.join(" "));
    Ok(full_config)
}
