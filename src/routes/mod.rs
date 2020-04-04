//! The rocket routes for interacting with the system

use crate::guard::auth::Auth;

use crate::{config::Config, responder::dor::DOR, templates::page::IndexTemplate, database::Database};
use rocket::{
    http::{ContentType, Status},
    response::content::Content,
    State,
};
use rust_embed::RustEmbed;
use std::{fs, io::ErrorKind, path::PathBuf, convert::TryInto};

pub mod auth;
pub mod catcher;
pub mod link;
pub mod upload;

/// The main page
#[get("/")]
pub fn index<'r>(
    config: State<'r, Config>,
    database: Database,
    auth: Option<Auth<'r>>,
) -> Result<DOR<'r, IndexTemplate<'r>>, Status> {
    match auth {
        None => Ok(DOR::login()),
        Some(_) => {
            let uploads = database.uploads().get_all_uploads().map_err(|e| {
                error!("Error indexing uploads: {}", e);

                Status::InternalServerError
            })?;
            let links = database.links().get_all_links().map_err(|e| {
                error!("Error indexing links: {}", e);

                Status::InternalServerError
            })?;

            Ok(DOR::data(IndexTemplate {
                config: config.inner(),
                link_count: links.len().try_into().map_err(|e| {
                    error!("Error converting usize to u64: {}", e);
    
                    Status::InternalServerError
                })?,
                upload_count: uploads.len().try_into().map_err(|e| {
                    error!("Error converting usize to u64: {}", e);
    
                    Status::InternalServerError
                })?,
                space_count: uploads.iter().fold(0, |acc, b| acc + b.size),
                total_hits: links.iter().fold(0, |acc, (_, hits)| acc + hits)
            }))
        },
    }
}

/// The urls to download the uploader templates from
#[get("/sxcu/<filename>")]
pub fn uploaders<'r>(
    auth: Option<Auth>,
    filename: String,
) -> Result<DOR<'static, Content<String>>, Status> {
    if let None = auth {
        return Ok(DOR::login_and_return(uri!(uploaders: filename)));
    }

    match fs::read_to_string(format!("data/uploaders/{}", filename)) {
        Err(e) if e.kind() == ErrorKind::NotFound => Err(Status::NotFound),
        Err(e) => {
            error!("Error reading uploader file {} {}", filename, e);

            Err(Status::InternalServerError)
        }
        Ok(s) => Ok(DOR::data(Content(ContentType::JSON, s))),
    }
}

/// Endpoint to acces static files
#[get("/<filename..>", rank = 100)]
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
