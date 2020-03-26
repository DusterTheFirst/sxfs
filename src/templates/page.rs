//! Askama templates for user accessable pages

use crate::auth::Auth;
use askama::Template;

/// The template for the login page
#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    /// The site name for customization
    pub site_name: String,
}

/// The template for the homepage
#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    /// The site name for customization
    pub site_name: String,
    /// The authentication used to access the page
    pub auth: Auth,
}
