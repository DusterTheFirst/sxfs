//! Guard for retreving the content headers from the request

use derive_more::{AsRef, Deref};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

/// Guard for retreving the content length from a request
#[derive(Debug, AsRef, Deref)]
pub struct ContentLength(usize);

impl<'a, 'r> FromRequest<'a, 'r> for ContentLength {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Content-Length") {
            Some(length) => Outcome::Success(ContentLength(length.parse().unwrap())),
            _ => Outcome::Forward(()),
        }
    }
}
