use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use base64;

pub struct AuthGaurd(String);

#[derive(Debug)]
pub enum AuthGaurdError {
    Missing,
    Invalid,
    BadCount
}

impl AuthGaurd {
    pub fn is_valid(token: &str) -> bool {
        debug!("{}", token);
        let token = token.replace("Basic ", "");
        let vector = match base64::decode(&token) {
            Ok(vec) => vec,
            _ => return false
        };
        let string = match std::str::from_utf8(&vector) {
            Ok(string) => string,
            _ => return false
        };
        debug!("{}", string);

        let mut split = string.splitn(2, ':');

        let username: &str = match split.next() {
            Some(x) => x,
            _ => return false
        };
        let password: &str = match split.next() {
            Some(x) => x,
            _ => return false
        };

        debug!("Username: {}\nPassword: {}", username.to_lowercase(), password);

        true
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthGaurd {
    type Error = AuthGaurdError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let tokens: Vec<_> = request.headers().get("Authorization").collect();

        match tokens.len() {
            0 => Outcome::Failure((Status::Unauthorized, AuthGaurdError::Missing)),
            1 if Self::is_valid(tokens[0]) => Outcome::Success(AuthGaurd(tokens[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, AuthGaurdError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, AuthGaurdError::BadCount)),
        }
    }
}