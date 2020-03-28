//! System-wide Identifier

use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::{self, UriDisplay, UriPart},
        RawStr,
    },
    request::FromParam,
};
use std::convert::TryFrom;
use std::{convert::TryInto, fmt, fmt::Display, ops::Deref};
use uuid::Uuid;

/// An identifier for a unit in the system
#[derive(Debug, Default)]
pub struct ID([u8; 16]);

impl_from_uri_param_identity!(ID);

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

impl AsRef<[u8]> for ID {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl<'r> FromParam<'r> for ID {
    type Error = anyhow::Error;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param.as_str().try_into()
    }
}

impl<P: UriPart> UriDisplay<P> for ID {
    fn fmt(&self, f: &mut uri::Formatter<P>) -> fmt::Result {
        f.write_value(base64::encode_config(self, base64::URL_SAFE_NO_PAD))
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", base64::encode_config(self, base64::URL_SAFE_NO_PAD))
    }
}

impl TryFrom<&str> for ID {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ID(
            base64::decode_config(value, base64::URL_SAFE_NO_PAD)?[..].try_into()?
        ))
    }
}
