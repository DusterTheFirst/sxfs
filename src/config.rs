//! The app wide configuration and tools to assist with manipulating it

use crate::generate::generate_base64;
use crate::templates::ConfigTemplate;
use askama::Template;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
/// The configuration for the app
pub struct Config {
    /// The name of the app to use
    pub name: String,
    /// If https is enabled for the site (behind reverse-proxy)
    pub https: bool,
    /// The domain to use for accessing and viewing the uploads
    pub domain: String,
    /// The domain to use for uploads (default: domain)
    ///
    /// **Only required if you need to bypass a upload limit like
    /// with cloudflare to use a different domain for direct comms
    /// to the server, bypassing cloudflare**
    pub upload_domain: Option<String>,
    /// The token to use for uploading files from sharex
    /// (Treat this like a password, it is an all access pass to upload)
    pub upload_token: String,
    /// The users to have access to the files
    pub users: HashMap<String, String>,
}

impl Config {
    /// Load a config file from the filesystem or create one based on the template
    /// 
    /// # Errors
    /// - If there is a problem reading the file
    /// - If there is a problem parsing the file
    pub fn load(path: &Path) -> anyhow::Result<Config> {
        // Check if path exixts
        if !path.exists() {
            debug!("Creating config file {:?} from template", path);
            // Write template if file does not exist
            fs::write(
                path,
                ConfigTemplate {
                    upload_token: &generate_base64(100),
                    admin_password: &generate_base64(25),
                }
                .render()?,
            )?;
        }

        trace!("Reading file contents from config file {:?}", path);
        // Parse in the toml config file
        Ok(toml::from_str::<Config>(&fs::read_to_string(path)?)?)
    }
}

/// Test that the template config file is a valid config file
#[test]
fn text_config_template() {
    // Parse the config as a config struct
    toml::from_str::<Config>(
        &fs::read_to_string("templates/config.toml").expect("Config template missing"),
    )
    .expect("Invalid toml");
}
