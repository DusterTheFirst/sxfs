//! Tools for authenticating users and tokens

use crate::{config::Config, user::User};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};
use std::convert::TryInto;

/// A method of authentication
#[derive(Debug)]
pub enum Auth {
    /// A user account used for authentication
    User(User),
    /// An upload token used for authentication
    UploadToken(String),
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // Load the config from the state
        let config = request.guard::<State<Config>>().unwrap();

        // Check for an upload token header
        if let Some(token) = request.headers().get_one("X-Upload-Token") {
            // If the token matches that in the config, return success and auth type
            if config.upload_token == token {
                return Outcome::Success(Auth::UploadToken(token.into()));
            } else {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        }

        // If there was no token header, check the cookies
        match request.cookies().get("auth") {
            // Decode the base64 encoded cookie if it exists
            Some(cookie) => match base64::decode_config(cookie.value(), base64::URL_SAFE) {
                Ok(val) => {
                    // Convert the raw bytes into a string
                    let val = String::from_utf8_lossy(&val);

                    // Split out the username and password from the cookie by a ":"
                    let [username, password]: [&str; 2] =
                        match val.split(':').take(2).collect::<Vec<_>>()[0..2].try_into() {
                            Ok(v) => v,
                            // Fail if it was unable to split it out
                            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
                        };

                    // Construct the user object
                    let user = User {
                        username: username.into(),
                        password: password.into(),
                    };

                    // Check if any of the users in the config match
                    if config
                        .users
                        .iter()
                        .map(|u| u.into())
                        .any(|u: User| u == user)
                    {
                        // Return the user in a success if so
                        Outcome::Success(Auth::User(user))
                    } else {
                        // Fail if the user did not exist
                        Outcome::Failure((Status::Unauthorized, ()))
                    }
                }
                // Fail if malformed base64
                Err(_) => Outcome::Failure((Status::BadRequest, ())),
            },
            // Forward request if no header or cookie
            None => Outcome::Forward(()),
        }
    }
}
