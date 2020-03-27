//! Catchers for errors caused by other routes

use crate::{
    config::Config,
    guard::auth::Auth,
    templates::error::{InternalErrorTemplate, PageNotFoundTemplate, UnauthorizedTemplate},
};
use rocket::{Request, State};

/// Catcher for when a page is not found and throws a 404
#[catch(404)]
pub fn not_found(req: &Request) -> PageNotFoundTemplate {
    PageNotFoundTemplate {
        uri: req.uri().path().into(),
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
        method: req.method().as_str().into(),
    }
}

/// Catcher for when a request is made to a protected resource and the user is not authorized
#[catch(401)]
pub fn unauthorized(req: &Request) -> UnauthorizedTemplate {
    UnauthorizedTemplate {
        uri: req.uri().path().into(),
        method: req.method().as_str().into(),
        reason: format!("{:?}", req.guard::<Auth>().failed()),
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
    }
}

/// Catcher for an irrivecoverable internal error
#[catch(500)]
pub fn internal_error(req: &Request) -> InternalErrorTemplate {
    InternalErrorTemplate {
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
    }
}
