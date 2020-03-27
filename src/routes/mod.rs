//! The rocket routes for interacting with the system

use crate::guard::auth::Auth;

use crate::{config::Config, responder::dor::DOR, templates::page::IndexTemplate};
use rocket::{http::ContentType, response::content::Content, State};
use rust_embed::RustEmbed;
use std::{fs, io::ErrorKind, path::PathBuf};

pub mod auth;
pub mod catcher;
pub mod link;
pub mod upload;

/// The main page
#[get("/")]
pub fn index(config: State<Config>, auth: Option<Auth>) -> DOR<IndexTemplate> {
    match auth {
        None => DOR::login(),
        Some(auth) => DOR::data(IndexTemplate {
            site_name: config.name.clone(),
            auth,
        }),
    }
}

/// The urls to download the uploader templates from
#[get("/sxcu/<filename>")]
pub fn uploaders<'r>(
    auth: Option<Auth>,
    filename: String,
) -> anyhow::Result<DOR<'static, Option<Content<String>>>> {
    if let None = auth {
        return Ok(DOR::login_and_return(uri!(uploaders: filename)));
    }

    match fs::read_to_string(format!("data/uploaders/{}", filename)) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(DOR::data(None)),
        Err(e) => Err(e.into()),
        Ok(s) => Ok(DOR::data(Some(Content(ContentType::JSON, s)))),
    }
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
