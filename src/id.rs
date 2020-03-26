//! System-wide Identifier

use rocket::{http::RawStr, request::FromParam};
use std::convert::TryFrom;
use std::{convert::TryInto, ops::Deref};
use uuid::Uuid;

/// An identifier for a unit in the system
#[derive(Debug, Default)]
pub struct ID([u8; 16]);

impl ID {
    /// Create a new random id
    #[must_use]
    pub fn new() -> ID {
        ID(*Uuid::new_v4().as_bytes())
    }

    /// Create an id from an existing UUID
    #[must_use]
    pub fn from(uuid: Uuid) -> ID {
        ID(*uuid.as_bytes())
    }
}

impl Deref for ID {
    type Target = [u8; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> FromParam<'r> for ID {
    type Error = anyhow::Error;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param.as_str().try_into()
    }
}

impl Into<String> for ID {
    fn into(self) -> String {
        base64::encode_config(*self, base64::URL_SAFE)
    }
}

impl TryFrom<&str> for ID {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ID(
            base64::decode_config(value, base64::URL_SAFE)?[0..16].try_into()?
        ))
    }
}
