#![allow(renamed_and_removed_lints, clippy::unknown_clippy_lints)]

use serde::{Serialize, Serializer};
use std::fmt;
use std::{num::TryFromIntError, path::StripPrefixError, string::FromUtf8Error};
use thiserror::Error;

///
pub type Result<T, E = Error> = std::result::Result<T, E>;

///
#[allow(dead_code)]
#[derive(Error)]
pub enum Error {
    ///
    #[error("`{0}`")]
    Generic(String),

    ///
    #[error("git: no head found")]
    NoHead,

    ///
    #[error("git: conflict during rebase")]
    RebaseConflict,

    ///
    #[error("git: remote url not found")]
    UnknownRemote,

    ///
    #[error("git: inconclusive remotes")]
    NoDefaultRemoteFound,

    ///
    #[error("git: work dir error")]
    NoWorkDir,

    ///
    #[error("git: uncommitted changes")]
    UncommittedChanges,

    ///
    #[error("git: can\u{2019}t run blame on a binary file")]
    NoBlameOnBinaryFile,

    ///
    #[error("binary file")]
    BinaryFile,

    ///
    #[error("io error:{0}")]
    Io(#[from] std::io::Error),

    ///
    #[error("git error:{0}")]
    Git(#[from] git2::Error),

    ///
    #[error("strip prefix error: {0}")]
    StripPrefix(#[from] StripPrefixError),

    ///
    #[error("utf8 error:{0}")]
    Utf8Conversion(#[from] FromUtf8Error),

    ///
    #[error("TryFromInt error:{0}")]
    IntConversion(#[from] TryFromIntError),

    #[error("serde_json error:{0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("structopt::clap error:{0}")]
    ClapError(#[from] structopt::clap::Error),

    // #[error("EasyCast error:{0}")]
    // EasyCast(#[from] easy_cast::Error),

    // #[error("shellexpand error:{0}")]
    // Shell(#[from] shellexpand::LookupError<std::env::VarError>),
    ///
    #[error("path string error")]
    PathString,
}

///
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        Self::Generic(format!("poison error: {error}"))
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(error: std::sync::mpsc::SendError<T>) -> Self {
        Self::Generic(format!("send error: {error}"))
    }
}

// impl<T> From<crossbeam_channel::SendError<T>> for Error {
//     fn from(error: crossbeam_channel::SendError<T>) -> Self {
//         Self::Generic(format!("send error: {error}"))
//     }
// }

// TODO - errror class name
///
impl Serialize for Error {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Debug for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "Error ({})", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::init_log;

    #[test]
    fn test_error_serialze() {
        init_log();
        let e = Error::NoHead;
        let serialized = serde_json::to_string(&e).unwrap();
        let message = format!("\"{}\"", e.to_string());

        assert_eq!(message, serialized);
        log::trace!("{}", serialized);
    }
}
