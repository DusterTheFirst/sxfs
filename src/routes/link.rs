//! Routes for handling shortened links

use crate::{
    config::Config,
    database::{Database, Link},
    guard::auth::Auth,
    id::ID,
    responder::dor::DOR,
    templates::page::{DeletedTemplate, LinksTemplate},
};
use chrono::Local;
use rocket::{
    http::{uri::Uri, Status},
    response::Redirect,
    State,
};
use rocket_contrib::json::Json;
use serde::Serialize;

/// The responded result to a successful link shorten
#[derive(Serialize)]
pub struct LinkResult {
    id: ID,
}

/// Endpoint to shorten a url
#[post("/l?<uri>")]
pub fn create(_auth: Auth, database: Database, uri: String) -> Result<Json<LinkResult>, Status> {
    match Uri::parse(&uri) {
        Ok(_) => {
            let link = Link {
                id: ID::new(),
                uri,
                timestamp: Local::now().naive_local(),
            };

            database.links().save_link(&link).map_err(|e| {
                error!(
                    "Error saving link: ID: {} Uri: {} Error: {}",
                    link.id, link.uri, e
                );

                Status::InternalServerError
            })?;

            Ok(Json(LinkResult { id: link.id }))
        }
        Err(e) => {
            warn!("Attempted to shorten invalid uri: {}", e);

            Err(Status::BadRequest)
        }
    }
}

/// Endpoint to view shortened urls
#[get("/l")]
pub fn all<'r>(
    auth: Option<Auth<'r>>,
    config: State<'r, Config>,
    database: Database,
) -> Result<DOR<'r, LinksTemplate<'r>>, Status> {
    Ok(match auth {
        Some(_) => DOR::data(LinksTemplate {
            links: database.links().get_all_links().map_err(|e| {
                error!("Error indexing links: {}", e);

                Status::InternalServerError
            })?,
            config: config.inner(),
        }),
        None => DOR::login_and_return(uri!(all)),
    })
}

/// Endpoint to use a shortened link
#[get("/l/<id>")]
pub fn follow(database: Database, id: ID) -> Result<Redirect, Status> {
    let links = database.links();
    match links.get_link(&id) {
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(Status::NotFound),
        Err(e) => {
            error!("Error fetching file metadata: ID: {} Error: {}", id, e);

            Err(Status::InternalServerError)
        }
        Ok((link, _)) => {
            links.hit(&id).map_err(|e| {
                error!("Error incrementing hits on link: ID: {} Error: {}", id, e);

                Status::InternalServerError
            })?;
            Ok(Redirect::to(link.uri))
        }
    }
}

/// Endpoint to delete a shortened link
#[get("/l/d/<id>")]
pub fn delete<'r>(
    database: Database,
    config: State<'r, Config>,
    auth: Option<Auth<'r>>,
    id: ID,
) -> Result<DOR<'r, DeletedTemplate<'r>>, Status> {
    match auth {
        Some(_) => match database.links().get_link(&id) {
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(Status::NotFound),
            Err(e) => {
                error!("Error fetching file link: ID: {} Error: {}", id, e);

                Err(Status::InternalServerError)
            }
            Ok((link, _)) => match database.links().delete_link(&id) {
                Err(e) => {
                    error!(
                        "Error deleting link: ID: {} Uri: {} Error: {}",
                        id, link.uri, e
                    );

                    Err(Status::InternalServerError)
                }
                Ok(()) => Ok(DOR::data(DeletedTemplate {
                    config: config.inner(),
                    resource_type: "link"
                })),
            },
        },
        None => Ok(DOR::login_and_return(uri!(delete: id))),
    }
}
