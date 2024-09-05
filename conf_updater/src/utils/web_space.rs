use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use tracing::warn;
use uuid::Uuid;

use conf_updater_common::ApiFailure;

use crate::config::{Config, MiscConfig};
use crate::utils::{ProgramError, try_from_utf8};

/// Determine the webroot of a website
///
/// The `user_uuid` is a valid part of a path, since the web root is defined as
/// `{conf_webroot}/{user_uuid.as_hyphenated()}/{website_uuid.as_hyphenated()}`,
/// where `conf_webroot` is configured globally and must be an absolute path.
pub(crate) fn get_webroot(
    user_uuid: &Uuid,
    website_uuid: &Uuid,
    web_conf: &MiscConfig,
) -> PathBuf {
    PathBuf::from(&web_conf.htdocs_root_dir)
        .join(user_uuid.as_hyphenated().to_string())
        .join(website_uuid.as_hyphenated().to_string())
}

/// Create the web root for a new site
///
/// Exit cleanly if the web root directory already exists and contains an index file.
/// The `user_id` is a valid part of a path, since the web root's index file is
/// expected at `{webroot}/index.html`, where `webroot` is the root of the user's webspace.
pub(crate) fn create_web_space(
    user_posix_id: u32,
    user_uuid: &Uuid,
    website: &Uuid,
    conf: &Config,
) -> Result<(), ApiFailure> {
    let web_root = get_webroot(user_uuid, website, &conf.misc);
    let index_path = web_root.join("index.html");
    fs::create_dir_all(&web_root)?;

    // If the index file does not exist, it can be created and filled with a simple default welcome page
    match File::create_new(index_path) {
        Ok(mut file) => {
            // Since Rust requires string literals in the format! macro,
            // these blocks of text need to be inlined into this function
            file.write(format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Start page</title>
</head>
<body>
    <h1>Willkommen</h1>
    <p>
        Willkommen auf Ihrer neuen Homepage zur Verfügung gestellt von <a href="{hosting_href}">{hosting_name}</a>.
    </p>
    <p>
        Wenn Sie diese Seite sehen können, wurde der Webserver erfolgreich für Sie vorbereitet.
    </p>
    <p>
        Als Nächstes müssen Sie ihren Inhalt zur Webseite hinzufügen.
        Dazu können Sie sich in ihren neuen Webspace unter <a href="{webspace_login}">{webspace_login}</a> einloggen.
        Löschen oder bearbeiten Sie diese Datei mit dem Namen <code>index.html</code> und beginnen Sie mit dem Bau ihrer neuen Website.
    </p>
    <p>
        Falls Sie Hilfe benötigen, zögern Sie nicht uns unter <a href="mailto:{hosting_help_email}">{hosting_help_email}</a> zu kontaktieren.
    </p>
    <hr/>
    <h1>Welcome</h1>
    <p>
        Welcome to your new homepage hosted by <a href="{hosting_href}">{hosting_name}</a>.
    </p>
    <p>
        If you see this page, the web server was successfully configured for you.
    </p>
    <p>
        You now need to add some content to this website.
        You can login to your new web space at <a href="{webspace_login}">{webspace_login}</a>.
        Delete or edit this file named <code>index.html</code> and start building your website.
    </p>
    <p>
        In case you need help, do not hesitate to contact us at <a href="mailto:{hosting_help_email}">{hosting_help_email}</a>.
    </p>
</body>
</html>
"#,
                hosting_href = &conf.hosting.website,
                hosting_name = &conf.hosting.name,
                webspace_login = &conf.hosting.webspace_login,
                hosting_help_email = &conf.hosting.help_address,
            ).as_bytes())?;
            file.sync_all()?;
        }
        Err(e) => { if e.kind() != AlreadyExists { warn!("While creating web space for {}: {}", website.as_hyphenated(), e) } }
    };
    set_permissions_recursively(&conf.misc)?;
    set_webspace_owner_recursively(user_posix_id, &web_root)?;
    set_webroot_group_recursively(&conf.misc)?;
    Ok(())
}

/// Recursively change the permissions of the web root to `-rw-r-----`
/// (mode 0640) for files and `drwxr-x---` (mode 0750) for directories
fn set_permissions_recursively(misc_conf: &MiscConfig) -> Result<(), ProgramError> {
    let output_files = Command::new("find")
        .arg(&misc_conf.htdocs_root_dir)
        .arg("-type")
        .arg("f")
        .arg("-exec")
        .arg("chmod 640 {} +")
        .output()?;
    if output_files.status.success() {
        let output_directories = Command::new("find")
            .arg(&misc_conf.htdocs_root_dir)
            .arg("-type")
            .arg("d")
            .arg("-exec")
            .arg("chmod 750 {} +")
            .output()?;
        if output_directories.status.success() {
            Ok(())
        } else {
            let stderr = try_from_utf8("webspace-find-chmod-dirs".to_string(), &*output_directories.stderr)?;
            Err(ProgramError::Failure("webspace-find-chmod-dirs".to_string(), stderr))
        }
    } else {
        let stderr = try_from_utf8("webspace-find-chmod-files".to_string(), &*output_files.stderr)?;
        Err(ProgramError::Failure("webspace-find-chmod-files".to_string(), stderr))
    }
}

/// Recursively set the owner of all files and directories in the webspace
fn set_webspace_owner_recursively(owner_uid: u32, directory: &PathBuf) -> Result<(), ProgramError> {
    let output = Command::new("chown")
        .arg("-R")
        .arg("--preserve-root") // safety measure
        .arg(owner_uid.to_string())
        .arg(directory.as_os_str())
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = try_from_utf8("webspace-find-chown".to_string(), &*output.stderr)?;
        Err(ProgramError::Failure("webspace-find-chown".to_string(), stderr))
    }
}

/// Recursively change the group ownership of the web root to the web group by calling `chgrp`
fn set_webroot_group_recursively(misc_conf: &MiscConfig) -> Result<(), ProgramError> {
    let output = Command::new("chgrp")
        .arg("-R")
        .arg(&misc_conf.nginx_group)
        .arg(&misc_conf.htdocs_root_dir)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = try_from_utf8("webspace-find-chgrp".to_string(), &*output.stderr)?;
        Err(ProgramError::Failure("webspace-find-chgrp".to_string(), stderr))
    }
}
