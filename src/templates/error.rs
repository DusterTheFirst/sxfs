//! HTML esrror page askama templates

use askama::Template;

/// Template for a 401 unauthorized error
#[derive(Template)]
#[template(path = "pages/errors/401.html")]
pub struct UnauthorizedTemplate {
    /// The method that was attempted
    pub method: String,
    /// The URI that was accessed
    pub uri: String,
    /// The reason for the lack of auth
    pub reason: String,
    /// The customized site name for display
    pub site_name: String,
}

/// Template for a 404 page not found error
#[derive(Template)]
#[template(path = "pages/errors/404.html")]
pub struct PageNotFoundTemplate {
    /// Method that was used to access the resource
    pub method: String,
    /// The missing resource
    pub uri: String,
    /// The customized site name for display
    pub site_name: String,
}

/// Template for a 500 internal error
#[derive(Template)]
#[template(path = "pages/errors/500.html")]
pub struct InternalErrorTemplate {
    /// The customized site name for diplay
    pub site_name: String,
}
