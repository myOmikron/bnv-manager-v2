//! Wrapper around [String] that requires not to be empty

use std::ops::Deref;

use schemars::JsonSchema;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use thiserror::Error;

/// Wrapper around [String] that requires not to be empty
#[derive(Debug, Clone, JsonSchema)]
pub struct NotEmptyString(String);

impl<'de> Deserialize<'de> for NotEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        if str.is_empty() {
            return Err(Error::custom("Empty string is not allowed here"));
        }

        Ok(NotEmptyString(str))
    }
}

impl Serialize for NotEmptyString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl NotEmptyString {
    /// Construct a new [NotEmptyString]
    ///
    /// # Errors
    /// when receiving an empty input string
    pub fn new(input: String) -> Result<Self, EmptyNotAllowed> {
        if input.is_empty() {
            return Err(EmptyNotAllowed);
        }

        Ok(NotEmptyString(input))
    }

    /// Convert in inner type
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for NotEmptyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Empty string found
#[derive(Debug, Error)]
#[error("Empty string is not allowed")]
pub struct EmptyNotAllowed;
