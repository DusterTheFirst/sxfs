//! Catchers for errors caused by other routes

use crate::{
    config::Config,
    templates::error::{InternalErrorTemplate, PageNotFoundTemplate, UnauthorizedTemplate},
};
use rocket::{Request, State};

/// Catcher for when a page is not found and throws a 404
#[catch(404)]
pub fn not_found(req: &Request) -> PageNotFoundTemplate {
    PageNotFoundTemplate {
        uri: req.uri().path().into(),
        config: req.guard::<State<Config>>().unwrap().inner().clone(),
        method: req.method().to_string(),
    }
}

/// Catcher for when a request is made to a protected resource and the user is not authorized
#[catch(401)]
pub fn unauthorized(req: &Request) -> UnauthorizedTemplate {
    UnauthorizedTemplate {
        uri: req.uri().path().into(),
        method: req.method().to_string(),
        config: req.guard::<State<Config>>().unwrap().inner().clone(),
    }
}

/// Catcher for an irrivecoverable internal error
#[catch(500)]
pub fn internal_error(req: &Request) -> InternalErrorTemplate {
    InternalErrorTemplate {
        config: req.guard::<State<Config>>().unwrap().inner().clone(),
    }
}
