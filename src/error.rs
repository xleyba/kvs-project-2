use failure::Fail;
use std::io;

pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Removing non-existent key error
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}