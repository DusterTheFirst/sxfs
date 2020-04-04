//! A responder that can contain data or a redirect

use crate::routes::auth::rocket_uri_macro_login_form;
use rocket::{
    http::uri::Uri,
    response::{self, Redirect, Responder},
    Request,
};
use std::marker::PhantomData;

/// A response that can contain either data or a redirect
pub enum DataOrRedirect<'r, T: Responder<'r>> {
    /// The resource that has been requested
    Data(Box<T>, PhantomData<&'r T>),
    /// A redirect to another page
    Redirect(Redirect),
}

impl<'r, T: Responder<'r>> Responder<'r> for DataOrRedirect<'r, T> {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            Self::Data(d, _) => d.respond_to(request),
            Self::Redirect(r) => r.respond_to(request),
        }
    }
}

impl<'r, T: Responder<'r>> DataOrRedirect<'r, T> {
    /// Response data
    pub fn data(value: T) -> Self {
        Self::Data(Box::new(value), PhantomData::default())
    }

    /// A redirect to the login form that will put the user back to the current_uri after the auth
    pub fn login_and_return<U: Into<Uri<'r>>>(current_uri: U) -> Self {
        Self::Redirect(Redirect::to(
            uri!(login_form: current_uri.into().to_string()),
        ))
    }

    /// A redirect to the login form
    pub fn login() -> Self {
        Self::Redirect(Redirect::to(uri!(login_form: _)))
    }

    /// A redirect to the given uri
    pub fn redirect<U: Into<Uri<'static>>>(uri: U) -> Self {
        Self::Redirect(Redirect::to(uri.into()))
    }
}

/// A short name for a DataOrRedirect responder
pub type DOR<'r, T> = DataOrRedirect<'r, T>;
