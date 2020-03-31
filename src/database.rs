//! Wrapper for the sql database as to provide storage
use crate::id::ID;
use chrono::NaiveDateTime;
use derive_more::Deref;
use rocket_contrib::database;
use rusqlite::Connection;

/// Wrapper for the sql database as to provide storage
#[database("db")]
#[derive(Debug, Deref)]
pub struct Database(Connection);

impl Database {
    /// Get the uploads table and methods to deal with it
    pub fn uploads(&self) -> UploadTable {
        UploadTable(&self)
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
    pub size: u32,
    /// The timestamp of when the upload was created
    pub timestamp: NaiveDateTime,
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
                size          NUMBER NOT NULL,
                timestamp     TEXT NOT NULL,
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
                &upload.size,
                &upload.timestamp,
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
                    size: row.get_checked(2)?,
                    timestamp: row.get_checked(3)?,
                })
            },
        )
    }

    /// Get an upload from the database, using its id
    pub fn get_upload_data(&self, id: &ID) -> rusqlite::Result<Box<UploadData>> {
        self.ensure_table_exists()?;

        self.query_row_and_then("SELECT contents FROM uploads WHERE id=?", &[id], |row| {
            Ok(row.get_checked::<usize, Vec<_>>(0)?.into_boxed_slice())
        })
    }

    /// Get all uploads from the database
    pub fn get_all_uploads(&self) -> rusqlite::Result<Box<[UploadMetadata]>> {
        self.ensure_table_exists()?;

        Ok(self
            .prepare("SELECT id, filename, size, timestamp FROM uploads")?
            .query_map::<rusqlite::Result<UploadMetadata>, _>(&[], |row| {
                Ok(UploadMetadata {
                    id: row.get_checked(0)?,
                    filename: row.get_checked(1)?,
                    size: row.get_checked(2)?,
                    timestamp: row.get_checked(3)?,
                })
            })?
            .flatten()
            .flatten()
            .collect())
    }
}