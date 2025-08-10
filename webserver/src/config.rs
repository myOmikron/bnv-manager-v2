//! Definitions of the configuration file

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::LazyLock;

use rorm::DatabaseDriver;
use webauthn_rs::prelude::Url;

use crate::config::env::EnvError;
use crate::config::env::EnvVar;

/// Load all environment variables declared in this module
///
/// Called at the beginning of `main` to gather and report all env errors at once.
pub fn load_env() -> Result<(), Vec<&'static EnvError>> {
    let mut errors = Vec::new();

    for result in [
        LISTEN_ADDRESS.load(),
        LISTEN_PORT.load(),
        ORIGIN.load(),
        STATE_DIR.load(),
        POSTGRES_HOST.load(),
        POSTGRES_DB.load(),
        POSTGRES_PORT.load(),
        POSTGRES_USER.load(),
        POSTGRES_PASSWORD.load(),
        MAILCOW_BASE_URL.load(),
        MAILCOW_API_KEY.load(),
        OTEL_EXPORTER_OTLP_ENDPOINT.load(),
    ] {
        errors.extend(result.err());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Address the API server should bind to
pub static LISTEN_ADDRESS: EnvVar<IpAddr> =
    EnvVar::optional("LISTEN_ADDRESS", || IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

/// Port the API server should bind to
pub static LISTEN_PORT: EnvVar<u16> = EnvVar::optional("LISTEN_PORT", || 8080);

/// The url this server is reachable under
///
/// # Used for
/// - generating links which should point back to bnv-manager
/// - defaulting [`WEBAUTHN_ID`] and [`WEBAUTHN_ORIGIN`]
pub static ORIGIN: EnvVar<Url> = EnvVar::required("ORIGIN");

/// A directory bnv-manager puts files it creates.
///
/// Most noteworthy, this will contain a `/media` where file uploaded by users will be stored.
pub static STATE_DIR: EnvVar<PathBuf> = EnvVar::optional("BNV_MANAGER_STATE_DIR", || {
    PathBuf::from("/var/lib/bnv-manager")
});

/// Mailcow base url
pub static MAILCOW_BASE_URL: EnvVar<Url> = EnvVar::required("MAILCOW_BASE_URL");

/// API key of the mailcow user
pub static MAILCOW_API_KEY: EnvVar = EnvVar::required("MAILCOW_API_KEY");

/// The address of the database server
pub static POSTGRES_HOST: EnvVar = EnvVar::optional("POSTGRES_HOST", || "postgres".to_string());

/// The database name
pub static POSTGRES_DB: EnvVar = EnvVar::required("POSTGRES_DB");

/// The port of the database server
pub static POSTGRES_PORT: EnvVar<u16> = EnvVar::optional("POSTGRES_PORT", || 5432);

/// The user to use for the database connection
pub static POSTGRES_USER: EnvVar = EnvVar::optional("POSTGRES_USER", || "postgres".to_string());

/// Password for the user
pub static POSTGRES_PASSWORD: EnvVar = EnvVar::optional("POSTGRES_PASSWORD", || "".to_string());

/// Bundle of all database variables combined in `rorm`'s format
pub static DB: LazyLock<DatabaseDriver> = LazyLock::new(|| DatabaseDriver::Postgres {
    name: POSTGRES_DB.clone(),
    host: POSTGRES_HOST.clone(),
    port: *POSTGRES_PORT,
    user: POSTGRES_USER.clone(),
    password: POSTGRES_PASSWORD.clone(),
});

/// The endpoint to export opentelemetry traces to
///
/// This variable is defined in the opentelemetry specifications and used implicitly by our dependencies.
/// It is declared explicitly here to be easier to discover and change its default.
pub static OTEL_EXPORTER_OTLP_ENDPOINT: EnvVar =
    EnvVar::optional("OTEL_EXPORTER_OTLP_ENDPOINT", || {
        "http://jaeger-dev:4317".to_string()
    });

mod env {
    use std::env;
    use std::env::VarError;
    use std::fmt;
    use std::ops::Deref;
    use std::sync::OnceLock;

    use serde::de::DeserializeOwned;
    use thiserror::Error;

    use super::deserializer::StringDeserializer;
    use super::deserializer::StringDeserializerError;

    /// An environment variable used to configure truffleport
    pub struct EnvVar<T = String> {
        value: OnceLock<Result<T, EnvError>>,

        name: &'static str,
        default: Option<fn() -> T>,
    }

    impl<T: DeserializeOwned> EnvVar<T> {
        /// Constructs an environment variable which is required
        pub const fn required(name: &'static str) -> Self {
            Self {
                name,

                value: OnceLock::new(),
                default: None,
            }
        }

        /// Constructs an environment variable which is optional and has a default
        pub const fn optional(name: &'static str, default: fn() -> T) -> Self {
            Self {
                name,

                value: OnceLock::new(),
                default: Some(default),
            }
        }

        /// Gets the environment variable's value (or its default)
        ///
        /// # Panics
        /// If the variable could not be read and parsed
        pub fn get(&self) -> &T {
            self.try_get().unwrap_or_else(|error| panic!("{error}"))
        }

        /// Loads the environment variable's value returning a possible error
        pub fn load(&self) -> Result<(), &EnvError> {
            self.try_get().map(|_| ())
        }

        /// Gets the environment variable's value (or its default)
        pub fn try_get(&self) -> Result<&T, &EnvError> {
            self.value
                .get_or_init(|| {
                    match env::var(self.name)
                        .map(StringDeserializer)
                        .map(T::deserialize)
                    {
                        Ok(Ok(value)) => Ok(value),
                        Ok(Err(StringDeserializerError(error))) => Err(EnvError {
                            name: self.name,
                            reason: EnvErrorReason::Malformed(error),
                        }),
                        Err(VarError::NotUnicode(_)) => Err(EnvError {
                            name: self.name,
                            reason: EnvErrorReason::NotUtf8,
                        }),
                        Err(VarError::NotPresent) => match self.default {
                            None => Err(EnvError {
                                name: self.name,
                                reason: EnvErrorReason::Missing,
                            }),
                            Some(default) => Ok(default()),
                        },
                    }
                })
                .as_ref()
        }
    }

    impl<T: DeserializeOwned> Deref for EnvVar<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.get()
        }
    }

    impl<T: DeserializeOwned + fmt::Display> fmt::Display for EnvVar<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.get().fmt(f)
        }
    }

    /// Error while reading and parsing an environment variable
    #[derive(Debug, Error, Clone)]
    #[error("Environment variable '{name}' is {reason}")]
    pub struct EnvError {
        /// The environment varible which cause this error
        pub name: &'static str,

        /// The reason why the environment variable couldn't be read
        pub reason: EnvErrorReason,
    }

    /// The reason why an environment variable couldn't be read
    #[derive(Debug, Error, Clone)]
    pub enum EnvErrorReason {
        /// Variable is not set
        #[error("not set")]
        Missing,

        /// Failed to decode the variable's value
        #[error("not utf8")]
        NotUtf8,

        /// Failed to parse the variable's value
        #[error("malformed: {0}")]
        Malformed(String),
    }
}

mod deserializer {
    use std::fmt::Display;

    use serde::Deserializer;
    use serde::de::Error;
    use serde::de::Visitor;
    use thiserror::Error;

    /// An improved [`StringDeserializer`](serde::de::value::StringDeserializer)
    pub struct StringDeserializer(pub String);

    /// Error produced by [`StringDeserializer`]
    #[derive(Debug, Error)]
    #[error("{0}")]
    pub struct StringDeserializerError(pub String);

    impl<'de> Deserializer<'de> for StringDeserializer {
        type Error = StringDeserializerError;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.0.as_str() {
                "true" | "1" | "yes" | "y" => visitor.visit_bool(true),
                "false" | "0" | "no" | "n" => visitor.visit_bool(false),
                _ => visitor.visit_string(self.0),
            }
        }

        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_i8(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_i16(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_i32(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_i64(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_u8(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_u16(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_u32(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_u64(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_f32(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_f64(self.0.parse().map_err(Self::Error::custom)?)
        }

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut chars = self.0.chars();
            if let Some(ch) = chars.next() {
                if chars.next().is_none() {
                    return visitor.visit_char(ch);
                }
            }
            visitor.visit_string(self.0)
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_some(self)
        }

        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_newtype_struct(self)
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_tuple_struct<V>(
            self,
            _name: &'static str,
            _len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }

        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_string(self.0)
        }
    }

    impl Error for StringDeserializerError {
        fn custom<T>(msg: T) -> Self
        where
            T: Display,
        {
            Self(msg.to_string())
        }
    }
}
