use crate::guard::auth::Auth;

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
#[post("/")]
pub fn create(_auth: Auth) -> String {
    unimplemented!();
}

/// Endpoint to view uploaded assets
#[get("/")]
pub fn all(_auth: Auth) -> String {
    unimplemented!();
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/<id>/<filename>")]
pub fn view(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    unimplemented!();
}

/// Endpoint to delete an uploaded assest by its ID and filename
#[get("/d/<id>/<filename>")]
pub fn delete(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    unimplemented!();
}
