use crate::guard::{auth::Auth, delete::Delete};
use anyhow::anyhow;
use rocket::{
    http::{uri::Uri, ContentType, Cookie, Cookies, RawStr, Status},
    request::Form,
    response::{content::Content, Redirect},
    Request, State,
};
use rust_embed::RustEmbed;
use std::{fs, io::ErrorKind, path::PathBuf};

use crate::{
    config::Config,
    id::ID,
    responder::dor::DOR,
    templates::{
        error::{InternalErrorTemplate, PageNotFoundTemplate, UnauthorizedTemplate},
        page::{IndexTemplate, LoginTemplate},
    },
    user::User,
};

/// The login form
#[get("/login?<redirect>")]
pub fn login_form(config: State<Config>, redirect: Option<String>) -> LoginTemplate {
    LoginTemplate {
        site_name: config.name.clone(),
        redirect: redirect.unwrap_or_else(|| "/".into()),
    }
}

/// The login submission portal
#[post("/login", data = "<user>")]
pub fn login_submit(mut cookies: Cookies, config: State<Config>, user: Form<User>) -> Status {
    // Check if the user submitted exixts
    if config
        .users
        .iter()
        .map(|u| u.into())
        .any(|u: User| u == *user)
    {
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