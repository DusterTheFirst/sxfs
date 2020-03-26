use crate::routes::auth::rocket_uri_macro_login_form;
use askama::rocket::Responder;
use rocket::{http::uri::Origin, response::Redirect, Request};
use std::marker::PhantomData;

/// A response that can contain either data or a redirect
pub enum DataOrRedirect<'r, T>
where
    T: Responder<'r>,
{
    /// The resource that has been requested
    Data(T, PhantomData<&'r T>),
    /// A redirect to another page
    Redirect(Redirect),
}

impl<'r, T: Responder<'r>> Responder<'r> for DataOrRedirect<'r, T> {
    fn respond_to(self, request: &Request) -> askama::rocket::Result<'r> {
        match self {
            Self::Data(d, _) => d.respond_to(request),
            Self::Redirect(r) => r.respond_to(request),
        }
    }
}

impl<'r, T: Responder<'r>> DataOrRedirect<'r, T> {
    /// Response data
    pub fn data(value: T) -> Self {
        Self::Data(value, PhantomData::default())
    }

    /// A redirect to the login form that will put the user back to the current_uri after the auth
    pub fn login_and_return<S: AsRef<str>>(current_uri: S) -> Self {
        Self::Redirect(Redirect::to(uri!(login_form: current_uri.as_ref())))
    }

    /// A redirect to the login form
    pub fn login() -> Self {
        Self::Redirect(Redirect::to(uri!(login_form: _)))
    }

    /// A redirect to the given uri
    pub fn redirect(uri: Origin<'static>) -> Self {
        Self::Redirect(Redirect::to(uri))
    }
}

/// A short name for a DataOrRedirect responder
pub type DOR<'r, T> = DataOrRedirect<'r, T>;
