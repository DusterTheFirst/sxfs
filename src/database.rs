//! Wrapper for the sql database as to provide storage
use crate::id::ID;
use chrono::NaiveDateTime;
use derive_more::Deref;
use rocket::http::ContentType;
use rocket_contrib::database;
use rusqlite::{types::FromSqlError, Connection};
use std::{convert::TryInto, path::Path};

/// Wrapper for the sql database as to provide storage
#[database("db")]
#[derive(Debug, Deref)]
pub struct Database(Connection);

impl Database {
    /// Get the uploads table and methods to deal with it
    pub fn uploads(&self) -> UploadTable {
        UploadTable(&self)
    }

    /// Get the links table and methods to deal with it
    pub fn links(&self) -> LinkTable {
        LinkTable(&self)
    }
}

/// Connection to an upload table
#[derive(Debug, Deref)]
pub struct UploadTable<'a>(&'a Connection);

/// An upload object that is stored in the upload table
pub struct UploadMetadata {
    /// The resource identifier for the upload
    pub id: ID,
    /// The filename for the stored upload
    pub filename: String,
    /// The size of the stored upload
    pub size: u64,
    /// The timestamp of when the upload was created
    pub timestamp: NaiveDateTime,
}

impl UploadMetadata {
    /// Helper fn to check if an upload is an image
    pub fn is_image(&self) -> bool {
        if let Some(ext) = Path::new(&self.filename).extension() {
            if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                ct.top() == "image"
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// The data that is stored in an upload
pub type UploadData = [u8];

impl<'a> UploadTable<'a> {
    /// Method to create the table if it does not exist
    fn ensure_table_exists(&self) -> rusqlite::Result<()> {
        self.execute(
            "CREATE TABLE IF NOT EXISTS uploads (
                id            BLOB PRIMARY KEY NOT NULL,
                filename      TEXT NOT NULL,
                size          BLOB NOT NULL,
                timestamp     NUMBER NOT NULL,
                contents      BLOB NOT NULL
            )",
            &[],
        )?;

        Ok(())
    }

    /// Save a new upload into the database
    pub fn save_upload(&self, upload: &UploadMetadata, data: &UploadData) -> rusqlite::Result<()> {
        self.ensure_table_exists()?;

        self.execute(
            "INSERT INTO uploads (id, filename, size, timestamp, contents) VALUES (?, ?, ?, ?, ?)",
            &[
                &upload.id,
                &upload.filename,
                &upload.size.to_ne_bytes().as_ref(),
                &upload.timestamp.timestamp(),
                &data,
            ],
        )?;

        Ok(())
    }

    /// Get an upload from the database, using its id
    pub fn get_upload_metatdata(&self, id: &ID) -> rusqlite::Result<UploadMetadata> {
        self.ensure_table_exists()?;

        self.query_row_and_then(
            "SELECT id, filename, size, timestamp FROM uploads WHERE id=?",
            &[id],
            |row| {
                Ok(UploadMetadata {
                    id: row.get_checked(0)?,
                    filename: row.get_checked(1)?,
                    size: u64::from_le_bytes(
                        (&row.get_checked::<_, Vec<u8>>(2)?[..])
                            .try_into()
                            .map_err(|e| {
                                error!("Error loading filesize: ID: {} Error: {}", id, e);

                                rusqlite::Error::FromSqlConversionFailure(
                                    0,
                                    rusqlite::types::Type::Blob,
                                    Box::new(FromSqlError::InvalidType),
                                )
                            })?,
                    ),
                    timestamp: NaiveDateTime::from_timestamp(row.get_checked(3)?, 0),
                })
            },
        )
    }

    /// Get an upload from the database, using its id
    pub fn get_upload_data(&self, id: &ID) -> rusqlite::Result<Box<UploadData>> {
        self.ensure_table_exists()?;

        self.query_row_and_then("SELECT contents FROM uploads WHERE id=?", &[id], |row| {
            Ok(row.get_checked::<_, Vec<_>>(0)?.into_boxed_slice())
        })
    }

    /// Get all uploads from the database
    pub fn get_all_uploads(&self) -> rusqlite::Result<Box<[UploadMetadata]>> {
        self.ensure_table_exists()?;

        Ok(self
            .prepare("SELECT id, filename, size, timestamp FROM uploads ORDER BY timestamp DESC")?
            .query_map::<rusqlite::Result<UploadMetadata>, _>(&[], |row| {
                Ok(UploadMetadata {
                    id: row.get_checked(0)?,
                    filename: row.get_checked(1)?,
                    size: u64::from_le_bytes(
                        (&row.get_checked::<_, Vec<u8>>(2)?[..])
                            .try_into()
                            .map_err(|e| {
                                error!("Error loading filesize: {}", e);

                                rusqlite::Error::FromSqlConversionFailure(
                                    0,
                                    rusqlite::types::Type::Blob,
                                    Box::new(FromSqlError::InvalidType),
                                )
                            })?,
                    ),
                    timestamp: NaiveDateTime::from_timestamp(row.get_checked(3)?, 0),
                })
            })?
            .flatten()
            .flatten()
            .collect())
    }

    /// Delete an existing upload
    pub fn delete_upload(&self, id: &ID) -> rusqlite::Result<()> {
        self.ensure_table_exists()?;

        self.execute("DELETE FROM uploads WHERE id=?", &[id])?;

        Ok(())
    }
}

/// Connection to the links table
#[derive(Debug, Deref)]
pub struct LinkTable<'a>(&'a Connection);

/// A link object that is stored in the link table
pub struct Link {
    /// The resource identifier for the link
    pub id: ID,
    /// The uri for the stored link to redirect to
    pub uri: String,
    /// The timestamp of when the link was created
    pub timestamp: NaiveDateTime,
}

/// The amount of hits on a link
pub type LinkHits = u32;

/// The type returned from listing a link
pub type LinkListing = (Link, LinkHits);

impl<'a> LinkTable<'a> {
    /// Method to create the table if it does not exist
    fn ensure_table_exists(&self) -> rusqlite::Result<()> {
        self.execute(
            "CREATE TABLE IF NOT EXISTS links (
                id          BLOB PRIMARY KEY NOT NULL,
                uri         TEXT NOT NULL,
                timestamp   NUMBER NOT NULL,
                hits        NUMBER NOT NULL
            )",
            &[],
        )?;

        Ok(())
    }

    /// Save a link into the database
    pub fn save_link(&self, link: &Link) -> rusqlite::Result<()> {
        self.ensure_table_exists()?;

        self.execute(
            "INSERT INTO links (id, uri, timestamp, hits) VALUES (?, ?, ?, 0)",
            &[&link.id, &link.uri.to_string(), &link.timestamp.timestamp()],
        )?;

        Ok(())
    }

    /// Get a link from the database, using its id
    pub fn get_link(&self, id: &ID) -> rusqlite::Result<LinkListing> {
        self.ensure_table_exists()?;

        self.query_row_and_then(
            "SELECT id, uri, timestamp, hits FROM links WHERE id=?",
            &[id],
            |row| {
                Ok((
                    Link {
                        id: row.get_checked(0)?,
                        uri: row.get_checked(1)?,
                        timestamp: NaiveDateTime::from_timestamp(row.get_checked(2)?, 0),
                    },
                    row.get_checked(3)?,
                ))
            },
        )
    }

    /// Get all links from the database
    pub fn get_all_links(&self) -> rusqlite::Result<Box<[LinkListing]>> {
        self.ensure_table_exists()?;

        Ok(self
            .prepare("SELECT id, uri, timestamp, hits FROM links ORDER BY timestamp DESC")?
            .query_map::<rusqlite::Result<_>, _>(&[], |row| {
                Ok((
                    Link {
                        id: row.get_checked(0)?,
                        uri: row.get_checked(1)?,
                        timestamp: NaiveDateTime::from_timestamp(row.get_checked(2)?, 0),
                    },
                    row.get_checked(3)?,
                ))
            })?
            .flatten()
            .flatten()
            .collect())
    }

    /// Get the amount of views that a link has gotten
    pub fn hit(&self, id: &ID) -> rusqlite::Result<()> {
        self.ensure_table_exists()?;

        self.execute("UPDATE links SET hits = hits + 1 WHERE id=?", &[id])?;

        Ok(())
    }

    /// Delete an existing link
    pub fn delete_link(&self, id: &ID) -> rusqlite::Result<()> {
        self.ensure_table_exists()?;

        self.execute("DELETE FROM links WHERE id=?", &[id])?;

        Ok(())
    }
}
