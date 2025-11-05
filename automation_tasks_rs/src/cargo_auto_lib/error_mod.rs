// error_mod.rs

//! Error library for this crate using thiserror.
//!
//! I am using the crate thiserror to create an enum for all library errors.  
//! It mostly forwards the source "from" error.  
//! The library never writes to the screen, because it contains only the logic.  
//! Is the bin project that knows if it is CLI, TUI or GUI and it presents the errors to the user and developer.  
//! Then in the bin project I use the crate anyhow.  

/// Enum of possible errors from this library
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("InfallibleError: {0}")]
    InfallibleError(#[from] std::convert::Infallible),
    #[error("StdIoError: {0}")]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    InquireError(#[from] inquire::InquireError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    CargoTomlError(#[from] cargo_toml::Error),
    #[error(transparent)]
    PatternError(#[from] glob::PatternError),
    #[error(transparent)]
    GlobError(#[from] glob::GlobError),
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SemverError(#[from] semver::Error),

    #[error("{0}")]
    ErrorFromString(String),
    #[error("{0}")]
    ErrorFromStr(&'static str),
    //#[error("unknown error")]
    //UnknownError,
}

/// Result type alias with fixed LibError using thiserror
///
/// It makes simpler to write returns from functions.
pub type Result<T, E = Error> = core::result::Result<T, E>;
