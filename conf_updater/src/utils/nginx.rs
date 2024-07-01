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
