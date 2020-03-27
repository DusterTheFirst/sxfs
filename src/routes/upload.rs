//! Routes for handling uploads

use crate::{guard::auth::Auth, id::ID, responder::dor::DOR};

/// Endpoint to upload an asset
#[post("/u")]
pub fn create(_auth: Auth) -> String {
    todo!();
}

/// Endpoint to view uploaded assets
#[get("/u")]
pub fn all(auth: Option<Auth>) -> DOR<'static, String> {
    match auth {
        Some(auth) => todo!(),
        None => DOR::login_and_return(uri!(all))
    }
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/u/<id>/<filename>")]
pub fn view(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    todo!();
}

/// Endpoint to delete an uploaded assest by its ID and filename
#[get("/u/d/<id>/<filename>")]
pub fn delete(id: ID, filename: Option<String>) -> String {
    dbg!(id);
    dbg!(filename);
    todo!();
}
