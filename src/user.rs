//! Storage of user's info

use rocket::request::FromForm;

/// A user that has access to the system
#[derive(FromForm, Debug, PartialEq)]
pub struct User {
    /// The user's login username
    pub username: String,
    /// The user's login password
    pub password: String,
}
