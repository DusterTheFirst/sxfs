//! Routes for handling uploads

use crate::{guard::auth::Auth, id::ID, responder::dor::DOR};
use rocket::response::Redirect;

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
        None => DOR::login_and_return(uri!(all)),
    }
}

/// Endpoint to access an uploaded assest by its ID
#[get("/u/<id>")]
pub fn view_by_id(id: ID) -> Redirect {
    dbg!(id);
    todo!();
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/u/<id>/<filename>")]
pub fn view(id: ID, filename: String) -> String {
    dbg!(id);
    dbg!(filename);
    todo!();
}

/// Endpoint to delete an uploaded assest by its ID
#[get("/u/d/<id>", rank = 2)]
pub fn delete_by_id(id: ID) -> Redirect {
    dbg!(id);
    todo!();
}

/// Endpoint to delete an uploaded assest by its ID and filename
#[get("/u/d/<id>/<filename>")]
pub fn delete(auth: Option<Auth>, id: ID, filename: String) -> DOR<'static, String> {
    match auth {
        Some(auth) => {
            dbg!(id);
            dbg!(filename);
            dbg!(auth);
            todo!()
        }
        None => DOR::login_and_return(uri!(delete: id, filename)),
    }
}
