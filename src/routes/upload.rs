//! Routes for handling uploads

use crate::{
    database::{Database, UploadMetadata},
    guard::{auth::Auth, content::ContentLength},
    id::ID,
    responder::dor::DOR,
};
use chrono::Local;
use rocket::{
    http::{ContentType, Status},
    response::{Content, Redirect},
    Data,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::{convert::TryInto, path::PathBuf};

/// The responded result to a successful file upload
#[derive(Serialize)]
pub struct UploadResult {
    filename: String,
    id: ID,
}

/// Endpoint to upload an asset
#[post("/u?<filename>", data = "<upload>")]
pub fn create<'r>(
    _auth: Auth,
    database: Database,
    upload_size: Option<ContentLength>,
    filename: Option<String>,
    upload: Data,
) -> Result<Json<UploadResult>, Status> {
    // Get the upload filename or create one with unknown as the name
    let filename = filename.unwrap_or_else(|| "unknown".into());
    // Generate an ID for the upload
    let id = ID::new();
    // Get the upload size from the
    let upload_size = upload_size.map_or(0, |u| *u);

    info!("New Upload: {:?} {} {}", filename, id, upload_size);

    // Stream the data into a vec that should be preallocated with the correct size
    let mut data = Vec::with_capacity(upload_size);
    upload.stream_to(&mut data).map_err(|e| {
        error!("Error streaming upload: {}", e);

        Status::InternalServerError
    })?;

    info!(
        "New Upload: {} {} {}",
        data.len(),
        upload_size,
        data.len() == upload_size
    );

    let upload = UploadMetadata {
        id,
        filename,
        size: data.len().try_into().map_err(|_| Status::PayloadTooLarge)?,
        timestamp: Local::now().naive_local(),
    };

    // Save the upload into the database
    database
        .uploads()
        .save_upload(&upload, data.as_slice())
        .map_err(|e| {
            error!(
                "Error saving file: ID: {} Filename: {} Error: {}",
                upload.id, upload.filename, e
            );

            Status::InternalServerError
        })?;

    // Send the result
    Ok(Json(UploadResult {
        filename: upload.filename,
        id: upload.id,
    }))
}

/// Endpoint to view uploaded assets
#[get("/u")]
pub fn all(auth: Option<Auth>, database: Database) -> Result<DOR<'static, String>, Status> {
    Ok(match auth {
        Some(auth) => DOR::data(
            database
                .uploads()
                .get_all_uploads()
                .map_err(|e| {
                    error!("Error indexing uploads: {}", e);

                    Status::InternalServerError
                })?
                .into_iter()
                .map(|x| {
                    format!(
                        "ID: {}, Filename: {}, Timestamp: {}, Filesize {}",
                        x.id, x.filename, x.timestamp, x.size
                    )
                })
                .collect::<Box<_>>()
                .join("\n"),
        ),
        None => DOR::login_and_return(uri!(all)),
    })
}

/// Endpoint to access an uploaded assest by its ID
#[get("/u/<id>")]
pub fn view_by_id(database: Database, id: ID) -> Result<Redirect, Status> {
    match database.uploads().get_upload_metatdata(&id) {
        Err(e) => {
            error!("Error fetching file metadata: ID: {} Error: {}", id, e);

            Err(Status::InternalServerError)
        }
        Ok(meta) => Ok(Redirect::to(uri!(view: &id, meta.filename))),
    }
}

/// Endpoint to access an uploaded assest by its ID and filename
#[get("/u/<id>/<filename>")]
pub fn view<'r>(database: Database, id: ID, filename: String) -> Result<Content<Vec<u8>>, Status> {
    match database.uploads().get_upload_metatdata(&id) {
        Err(e) => {
            error!("Error fetching file metadata: ID: {} Error: {}", id, e);

            Err(Status::InternalServerError)
        }
        Ok(metadata) => {
            if metadata.filename == filename {
                let content_type = match PathBuf::from(filename).extension() {
                    Some(ext) => ContentType::from_extension(ext.to_string_lossy().as_ref())
                        .unwrap_or(ContentType::Binary),
                    None => ContentType::Binary,
                };

                match database.uploads().get_upload_data(&id) {
                    Err(e) => {
                        error!(
                            "Error fetching file data: ID: {} Filename: {} Error: {}",
                            id, metadata.filename, e
                        );

                        Err(Status::InternalServerError)
                    }
                    Ok(data) => Ok(Content(content_type, data.to_vec())),
                }
            } else {
                Err(Status::NotFound)
            }
        }
    }
}

/// Endpoint to delete an uploaded assest by its ID
#[get("/u/d/<id>", rank = 2)]
pub fn delete_by_id(id: ID) -> Redirect {
    dbg!(id);
    todo!();
}

/// Endpoint to delete an uploaded assest by its ID and filename
#[get("/u/d/<id>/<filename>")]
pub fn delete(auth: Option<Auth>, id: ID, filename: String) -> DOR<'static, String> {
    match auth {
        Some(auth) => {
            dbg!(id);
            dbg!(filename);
            dbg!(auth);
            todo!()
        }
        None => DOR::login_and_return(uri!(delete: id, filename)),
    }
}
