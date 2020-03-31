//! Routes for handling shortened links

use crate::{guard::auth::Auth, id::ID, responder::dor::DOR};
use rocket::http::{uri::{self, Uri}, RawStr};

/// Endpoint to shorten a url
#[post("/l?<url>")]
pub fn create(_auth: Auth, url: &RawStr) -> Result<String, uri::Error> {
    let url = Uri::parse(url)?;
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
