use std::io;
use std::process::Command;

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
pub(crate) fn reload_server() -> io::Result<Result<(), String>> {
    let output = Command::new("nginx").arg("-s").arg("reload").output()?;
    if output.status.success() {
        Ok(Ok(()))
    } else {
        let stderr = std::str::from_utf8(&*output.stderr).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "unexpected error parsing UTF-8 sequence from nginx reload call",
            )
        })?;
        Ok(Err(stderr.into()))
    }
}

/// Verify the current nginx web server configuration, returning errors on failure
///
/// This function either returns an IO error on general IO failure or the result
/// of the check, which could fail on its own. That's why there are nested results.
pub(crate) fn verify_config() -> io::Result<Result<(), String>> {
    let output = Command::new("nginx").arg("-t").output()?;
    if output.status.success() {
        Ok(Ok(()))
    } else {
        let stderr = std::str::from_utf8(&*output.stderr).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "unexpected error parsing UTF-8 sequence from nginx config check",
            )
        })?;
        Ok(Err(stderr.into()))
    }
}
