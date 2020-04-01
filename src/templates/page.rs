//! Askama templates for user accessable pages

use crate::{
    config::Config,
    database::{LinkListing, UploadMetadata},
    guard::auth::Auth,
};
use askama::Template;

/// The template for the login page
#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The url to redirect to after a successful login
    pub redirect: String,
}

/// The template for the homepage
#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The authentication used to access the page
    pub auth: Auth<'a>,
}

/// The template for the uploads page
#[derive(Template)]
#[template(path = "pages/uploads.html")]
pub struct UploadsTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The authentication used to access the page
    pub auth: Auth<'a>,
    /// The upload metadata to list
    pub uploads: Box<[UploadMetadata]>,
}

/// The template for the uploads page
#[derive(Template)]
#[template(path = "pages/links.html")]
pub struct LinksTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The authentication used to access the page
    pub auth: Auth<'a>,
    /// The upload metadata to list
    pub links: Box<[LinkListing]>,
}
