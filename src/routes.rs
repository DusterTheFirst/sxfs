//! The rocket routes for interacting with the system

use crate::auth::Auth;
use rocket::{
    http::{ContentType, Cookie, Cookies, Status},
    request::Form,
    response::{content::Content, Redirect},
    Request, State,
};
use rust_embed::RustEmbed;
use std::path::PathBuf;
use url::Url;

use crate::{
    config::Config,
    id::ID,
    templates::{
        error::{InternalErrorTemplate, PageNotFoundTemplate, UnauthorizedTemplate},
        page::{IndexTemplate, LoginTemplate},
    },
    user::User,
};

/// Catcher for when a page is not found and throws a 404
#[catch(404)]
pub fn not_found(req: &Request) -> PageNotFoundTemplate {
    PageNotFoundTemplate {
        uri: req.uri().path().into(),
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
        method: req.method().as_str().into(),
    }
}

/// Catcher for when a request is made to a protected resource and the user is not authorized
#[catch(401)]
pub fn unauthorized(req: &Request) -> UnauthorizedTemplate {
    UnauthorizedTemplate {
        uri: req.uri().path().into(),
        method: req.method().as_str().into(),
        reason: format!("{:?}", req.guard::<Auth>().failed()),
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
    }
}

/// Catcher for an irrivecoverable internal error
#[catch(500)]
pub fn internal_error(req: &Request) -> InternalErrorTemplate {
    InternalErrorTemplate {
        site_name: req.guard::<State<Config>>().unwrap().name.clone(),
    }
}

/// The main page
#[get("/")]
pub fn index(config: State<Config>, auth: Auth) -> IndexTemplate {
    IndexTemplate {
        site_name: config.name.clone(),
        auth,
    }
}

/// The redirection to the login form in the case that the user is not authenticated
#[get("/", rank = 2)]
pub fn index_redirect() -> Redirect {
    Redirect::to(uri!(login_form))
}

/// The login form
#[get("/login")]
pub fn login_form(config: State<Config>) -> LoginTemplate {
    LoginTemplate {
        site_name: config.name.clone(),
    }
}

/// The logout flow
#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("auth"));

    Redirect::to("/")
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

/// The urls to download the uploader templates from
#[get("/uploaders/<filename>")]
pub fn uploaders(auth: Auth, filename: String) -> String {
    unimplemented!();
}

/// Endpoint to upload an asset
#[post("/upload")]
pub fn upload(auth: Auth) -> String {
    unimplemented!();
}

/// Endpoint to shorten a url
#[post("/shorten?<url>")]
pub fn shorten(auth: Auth, url: String) -> anyhow::Result<String> {
    let url = Url::parse(&url)?;
    dbg!(url);
    unimplemented!();
}

/// Endpoint to access an uploaded asset by ID only
#[get("/u/<id>")] // TODO: USE ROCKET_CONTRIB STATIC FOR SERVING
pub fn redirect_to_upload(id: ID) -> Redirect {
    dbg!(id);
    unimplemented!();
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/u/<id>/<filename>")]
pub fn view_upload(id: ID, filename: String) -> String {
    dbg!(id);
    dbg!(filename);
    unimplemented!();
}

/// Endpoint to use a shortened link
#[get("/l/<id>")]
pub fn redirect_short_link(id: ID) -> String {
    dbg!(id);
    unimplemented!();
}

/// Endpoint to acces static files
#[get("/<filename..>", rank = 10)]
pub fn public_files(filename: PathBuf) -> Option<Content<Vec<u8>>> {
    #[derive(RustEmbed)]
    #[folder = "public"]
    struct PublicFiles;

    // Try to load file from the public files
    match PublicFiles::get(&filename.to_string_lossy()) {
        Some(file) => Some(Content(
            // Get content type from extention
            ContentType::from_extension(
                &filename
                    .extension()
                    .map_or("txt".to_owned(), |e| e.to_string_lossy().into()),
            )
            .unwrap_or(ContentType::Plain),
            // Send the file
            file.into(),
        )),
        None => None,
    }
}
