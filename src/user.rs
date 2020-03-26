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

impl<S: AsRef<str>> From<(S, S)> for User {
    fn from((username, password): (S, S)) -> Self {
        User {
            username: username.as_ref().into(),
            password: password.as_ref().into(),
        }
    }
}
