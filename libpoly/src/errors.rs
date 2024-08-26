//! Error types for libpoly.

use thiserror::Error;

/// The main error type for libpoly
#[derive(Error, Debug)]
pub enum PolyRestError {
        #[error("error making HTTP request")]
        HttpError(#[from] reqwest::Error),

        #[error("error parsing response")]
        ParseError(#[from] serde_json::Error),

        #[error("error serializing data")]
        SerError(#[from] quick_xml::DeError),

        #[error("error sending message")]
        SendError(#[from] diqwest::error::Error),

        #[error("error fixing XML output")]
        XMLEscapeError(#[from] quick_xml::escape::EscapeError)
}