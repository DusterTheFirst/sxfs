//! Routes for handling shortened links

use crate::{guard::auth::Auth, id::ID, responder::dor::DOR};
use anyhow::anyhow;
use rocket::http::{uri::Uri, RawStr};

/// Endpoint to shorten a url
#[post("/l?<url>")]
pub fn create(_auth: Auth, url: &RawStr) -> anyhow::Result<String> {
    let url = Uri::parse(url).map_err(|e| anyhow!(e.to_string()))?;
    dbg!(url);
    todo!();
}

/// Endpoint to view shortened urls
#[get("/l")]
pub fn all<'a>(auth: Option<Auth>) -> DOR<'a, String> {
    match auth {
        Some(auth) => todo!(),
        None => DOR::login_and_return(uri!(all)),
    }
}

/// Endpoint to use a shortened link
#[get("/l/<id>")]
pub fn follow(id: ID) -> String {
    dbg!(id);
    todo!();
}

/// Endpoint to delete a shortened link
#[get("/l/d/<id>")]
pub fn delete(id: ID) -> String {
    dbg!(id);
    todo!();
}
