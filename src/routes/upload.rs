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

/// Endpoint to upload an asset
#[post("/u")]
pub fn create(auth: Auth) -> String {
    unimplemented!();
}

/// Endpoint to view uploaded assets
#[get("/u")]
pub fn view_all(auth: Auth) -> String {
    unimplemented!();
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/u/<id>/<filename>")]
pub fn view(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    unimplemented!();
}

/// Endpoint to delete an uploaded assest by its ID and filename
#[get("/u/d/<id>/<filename>")]
pub fn delete(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    unimplemented!();
}
