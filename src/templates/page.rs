//! Askama templates for user accessable pages

use crate::{
    config::Config,
    database::{LinkListing, UploadMetadata},
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
    /// The amount of uploads on the site
    pub upload_count: u64,
    /// The total filesize of all the uploads
    pub space_count: u64,
    /// The amount of links on the site
    pub link_count: u64,
    /// The total hits on the links combined
    pub total_hits: u32
}

/// The template for the uploads page
#[derive(Template)]
#[template(path = "pages/uploads.html")]
pub struct UploadsTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The upload metadata to list with its index
    pub uploads: Box<[(usize, UploadMetadata)]>,
}

/// The template for the links page
#[derive(Template)]
#[template(path = "pages/links.html")]
pub struct LinksTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The upload metadata to list
    pub links: Box<[LinkListing]>,
}

/// The template for the deleted page
#[derive(Template)]
#[template(path = "pages/deleted.html")]
pub struct DeletedTemplate<'a> {
    /// The site configuration
    pub config: &'a Config,
    /// The type of resource that it was
    pub resource_type: &'a str,
}
