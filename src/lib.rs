//! Tools to assist the ShareX File Server

#![feature(proc_macro_hygiene, decl_macro, never_type)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use std::{fs, io, path::Path};

pub mod args;
pub mod config;
pub mod database;
pub mod generate;
pub mod guard;
pub mod id;
pub mod responder;
pub mod routes;
pub mod templates;
pub mod user;

/// Helper function to create the parent directories of a file
pub fn create_parent_directories<P: AsRef<Path>>(path: &P) -> io::Result<bool> {
    // Check if the path has parents
    if let Some(parent) = path.as_ref().parent() {
        // Create the parents if they do not exist
        if !parent.exists() {
            trace!("Creating parent directories for {:?}", path.as_ref());
            fs::create_dir_all(parent)?;

            return Ok(true);
        }
    };

    Ok(false)
}
