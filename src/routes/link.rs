use crate::guard::{auth::Auth, delete::Delete};
use anyhow::anyhow;
use rocket::{
    http::{uri::Uri, ContentType, Cookie, Cookies, RawStr, Status},
    request::Form,
    response::{content::Content, Redirect},
    Request, State,
};
use rust_embed::RustEmbed;
use std::{fs, io::ErrorKind, path::PathBuf};

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
#[post("/l?<url>")]
pub fn create(auth: Auth, url: &RawStr) -> anyhow::Result<String> {
    let url = Uri::parse(url).map_err(|e| anyhow!(e.to_string()))?;
    dbg!(url);
    unimplemented!();
}

/// Endpoint to view shortened url
#[get("/l")]
pub fn view_all(auth: Auth) -> anyhow::Result<String> {
    unimplemented!();
}

/// Endpoint to use a shortened link
#[get("/l/<id>")]
pub fn follow(id: ID) -> String {
    dbg!(id);
    unimplemented!();
}
