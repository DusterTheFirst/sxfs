//! HTML esrror page askama templates

use askama::Template;
use crate::config::Config;

/// Template for a 401 unauthorized error
#[derive(Template)]
#[template(path = "pages/errors/401.html")]
pub struct UnauthorizedTemplate {
    /// The method that was attempted
    pub method: String,
    /// The URI that was accessed
    pub uri: String,
    /// The site configuration
    pub config: Config,
}

/// Template for a 404 page not found error
#[derive(Template)]
#[template(path = "pages/errors/404.html")]
pub struct PageNotFoundTemplate {
    /// Method that was used to access the resource
    pub method: String,
    /// The missing resource
    pub uri: String,
    /// The site configuration
    pub config: Config,
}

/// Template for a 500 internal error
#[derive(Template)]
#[template(path = "pages/errors/500.html")]
pub struct InternalErrorTemplate {
    /// The site configuration
    pub config: Config,
}
