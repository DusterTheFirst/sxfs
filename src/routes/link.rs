use crate::guard::auth::Auth;
use anyhow::anyhow;
use rocket::{
    http::{uri::Uri, ContentType, Cookie, Cookies, RawStr, Status},
    request::Form,
    response::{content::Content, Redirect},
    Request, State,
};

use crate::{
    config::Config,
    id::ID,
    responder::dor::DOR,
    templates::{
        error::{InternalErrorTemplate, PageNotFoundTemplate, UnauthorizedTemplate},
        page::{IndexTemplate, LoginTemplate},
    },
    user::User,
};

/// Endpoint to shorten a url
#[post("/?<url>")]
pub fn create(_auth: Auth, url: &RawStr) -> anyhow::Result<String> {
    let url = Uri::parse(url).map_err(|e| anyhow!(e.to_string()))?;
    dbg!(url);
    unimplemented!();
}

/// Endpoint to view shortened url
#[get("/")]
pub fn all(_auth: Auth) -> anyhow::Result<String> {
    unimplemented!();
}

/// Endpoint to use a shortened link
#[get("/<id>")]
pub fn follow(id: ID) -> String {
    dbg!(id);
    unimplemented!();
}

/// Endpoint to delete a shortened link
#[get("/d/<id>")]
pub fn delete(id: ID) -> String {
    dbg!(id);
    unimplemented!();
}
