use std::io;
use std::process::Command;
use conf_updater_common::ApiFailure;

const NGINX_SERVER_BASE_TEMPLATE: &str = "# This file is managed by the web conf updater

server {{
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name {domain_names};

    root {root_dir};
    index index.html index.htm;

    location / {{
        try_files $uri $uri/ =404;
    }}

    ssl_certificate /etc/letsencrypt/live/{website_uuid}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/{website_uuid}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}}{forwarding_extension}";

const NGINX_SERVER_FORWARDING_EXTENSION_TEMPLATE: &str = "\n\nserver {{
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name {domain_names};

    root {root_dir};
    index index.html index.htm;

    location / {{
        return 302 https://{target_domain}/$request_uri;
    }}

    ssl_certificate /etc/letsencrypt/live/{website_uuid}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/{website_uuid}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}}";

const NGINX_SERVER_HTTP_EXTENSION_TEMPLATE: &str = "\n\nserver {
   listen 80;
   listen [::]:80;
   server_name {domain_names};
   return 301 https://$host$request_uri;
}";

/// Check that the `nginx` executable is found by `which`
pub(crate) fn check_available() -> bool {
    let result = Command::new("which").arg("bash").output();
    match result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Reload the nginx web server, returning the utility's response on error
pub(crate) fn reload_server() -> Result<(), ApiFailure> {
    let output = Command::new("nginx").arg("-s").arg("reload").output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&*output.stderr).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "unexpected error parsing UTF-8 sequence from nginx reload call",
            )
        })?;
        tracing::event!(tracing::Level::ERROR, "Failed to reload nginx server: {}", stderr);
        Err(ApiFailure::InternalServerError)
    }
}

/// Verify the current nginx web server configuration, returning errors on failure
pub(crate) fn verify_config() -> Result<(), ApiFailure> {
    let output = Command::new("nginx").arg("-t").output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&*output.stderr).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "unexpected error parsing UTF-8 sequence from nginx config check",
            )
        })?;
        tracing::event!(tracing::Level::ERROR, "Failed to verify nginx conf: {}", stderr);
        Err(ApiFailure::InternalServerError)
    }
}
