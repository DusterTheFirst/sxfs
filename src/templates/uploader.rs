//! Templates for Share X Custom Uploaders

use super::UpdatableTemplate;
use crate::config::Config;
use askama::Template;

/// Template for the uploader sxcu
#[derive(Template)]
#[template(path = "uploader.sxcu", escape = "none")]
pub struct UploaderTemplate<'a> {
    /// The name of the site/uploader
    pub name: &'a str,
    /// The domain to use for uploading
    pub upload_domain: &'a str,
    /// The token used to upload
    pub upload_token: &'a str,
    /// The domain for accessing the uploaded assets
    pub domain: &'a str,
    /// The protocol to use
    pub proto: &'a str,
}

impl<'a> UpdatableTemplate for UploaderTemplate<'a> {}
impl<'a> UploaderTemplate<'a> {
    /// Create a new uploader using the values from the config
    #[must_use]
    pub fn new(config: &'a Config) -> Self {
        UploaderTemplate {
            domain: &config.domain,
            name: &config.name,
            proto: if config.https { "https" } else { "http" },
            upload_domain: config.upload_domain.as_ref().unwrap_or(&config.domain),
            upload_token: &config.upload_token,
        }
    }
}

/// Tmeplate for the url shortener sxcu
#[derive(Template)]
#[template(path = "shortener.sxcu", escape = "none")]
pub struct ShortenerTemplate<'a> {
    /// The name of the shortener/site
    pub name: &'a str,
    /// The upload token for auth
    pub upload_token: &'a str,
    /// The domain to use for shortening
    pub domain: &'a str,
    /// The protocol to use
    pub proto: &'a str,
}

impl<'a> UpdatableTemplate for ShortenerTemplate<'a> {}
impl<'a> ShortenerTemplate<'a> {
    /// Create a new shortener from the values in the config
    #[must_use]
    pub fn new(config: &'a Config) -> Self {
        ShortenerTemplate {
            domain: &config.domain,
            name: &config.name,
            proto: if config.https { "https" } else { "http" },
            upload_token: &config.upload_token,
        }
    }
}
