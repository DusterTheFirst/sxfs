//! Routes for handling authentication

use crate::{
    config::Config, guard::auth::Auth, responder::dor::DOR, routes::rocket_uri_macro_index,
    templates::page::LoginTemplate, user::User,
};
use rocket::{
    http::{uri::Uri, Cookie, Cookies, Status},
    request::Form,
    response::Redirect,
    State,
};
use std::convert::TryInto;

/// The login form
#[get("/login?<redirect>")]
pub fn login_form<'r>(
    auth: Option<Auth<'r>>,
    config: State<'r, Config>,
    redirect: Option<String>,
) -> DOR<'r, LoginTemplate<'r>> {
    let redirect = redirect.unwrap_or_else(|| "/".into());
    match auth {
        Some(_) => DOR::redirect::<Uri<'static>>(redirect.try_into().unwrap_or_else(|_| uri!(index).into())),
        None => DOR::data(LoginTemplate {
            config: config.inner(),
            redirect,
        }),
    }
}

/// The login submission portal
#[post("/login", data = "<user>")]
pub fn login_submit(mut cookies: Cookies, config: State<Config>, user: Form<User>) -> Status {
    // Check if the user submitted exixts
    if config.users.iter().any(|u| *u == *user) {
        // If the user exists, add the cookie with their authentication information
        cookies.add(
            Cookie::build(
                "auth",
                base64::encode_config(
                    format!("{}:{}", user.username, user.password),
                    base64::URL_SAFE,
                ),
            )
            .permanent()
            .finish(),
        );

        // Return a successful status to the async loginer
        Status::Accepted
    } else {
        // Return a failure if the user does not exist
        Status::NotAcceptable
    }
}

/// The logout flow
#[get("/logout?<redirect>")]
pub fn logout(mut cookies: Cookies, redirect: Option<String>) -> Redirect {
    cookies.remove(Cookie::named("auth"));

    Redirect::to(redirect.unwrap_or_else(|| "/".into()))
}
