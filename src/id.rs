//! System-wide Identifier

use derive_more::{AsRef, Deref, Display};
use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::{self, UriDisplay, UriPart},
        RawStr,
    },
    request::FromParam,
};
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::Serialize;
use std::convert::TryFrom;
use std::{convert::TryInto, fmt, ops::Deref};
use uuid::Uuid;
use base64::DecodeError;

/// An identifier for a unit in the system
#[derive(Debug, Default, Deref, AsRef, Display)]
#[as_ref(forward)]
#[display(fmt = "{}", "String::from(self)")]
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

impl<'r> FromParam<'r> for ID {
    type Error = DecodeError;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param.as_str().try_into()
    }
}

impl<P: UriPart> UriDisplay<P> for ID {
    fn fmt(&self, f: &mut uri::Formatter<P>) -> fmt::Result {
        f.write_value(base64::encode_config(self, base64::URL_SAFE_NO_PAD))
    }
}

impl From<&ID> for String {
    fn from(src: &ID) -> String {
        base64::encode_config(src, base64::URL_SAFE_NO_PAD)
    }
}

impl TryFrom<&str> for ID {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ID(
            base64::decode_config(value, base64::URL_SAFE_NO_PAD)?[..].try_into().map_err(|_| DecodeError::InvalidLength)?,
        ))
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToSql for ID {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Blob(self.deref())))
    }
}

impl FromSql for ID {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        Ok(ID(value.as_blob()?.try_into().unwrap_or_default()))
    }
}
