//! Askama templates for generating files or responses from the server

use crate::create_parent_directories;
use askama::Template;
use io::ErrorKind;
use std::{fs, io, path::Path};

pub mod error;
pub mod page;
pub mod uploader;

/// A template that will be repeatedly written to disk
pub trait UpdatableTemplate: Template {
    /// Update the file to match the rendered template
    ///
    /// # Errors
    /// - Fails to create parent directories
    /// - Fails to renders
    /// - Fails to read existing file
    /// - Fails to write new file contents
    fn update<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        // Create the parent directories if they do not already exist
        create_parent_directories(&path)?;

        let new_content = self
            .render()
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

        // Only check for update if the file already exists
        if path.as_ref().exists() {
            // Write changes if there are any and alert of them
            if new_content == fs::read_to_string(path.as_ref())? {
                trace!("No changes in rendered template. Skipping write");
            } else {
                debug!("Updated rendered template at {:?}", path.as_ref());
                fs::write(path.as_ref(), new_content)?;
            }
        } else {
            // Create the file if it does not exist
            debug!("Creating rendered template at {:?}", path.as_ref());
            fs::write(path.as_ref(), new_content)?;
        }

        Ok(())
    }
}

/// Template for the default config
#[derive(Template)]
#[template(path = "config.toml", escape = "none")]
pub struct ConfigTemplate<'a> {
    /// A secure, custom upload token for this installation
    pub upload_token: &'a str,
    /// The secure, custom password to use for the default account
    pub admin_password: &'a str,
}
