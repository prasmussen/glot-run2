use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_json;
use std::io;
use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, Write};
use std::fmt;
use tempfile::NamedTempFile;


pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), WriteJsonError> {
    let dir = path.parent()
        .ok_or(WriteJsonError::DetermineDir())?;

    let file = NamedTempFile::new_in(dir)
        .map_err(WriteJsonError::CreateTempFile)?;

    serde_json::to_writer_pretty(&file, value)
        .map_err(WriteJsonError::Serialize)?;

    file.persist(path)
        .map_err(|err| WriteJsonError::Persist(err.error))?;

    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, ReadJsonError> {
    let file = File::open(path)
        .map_err(ReadJsonError::Open)?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(ReadJsonError::Deserialize)
}



pub enum WriteJsonError {
    DetermineDir(),
    CreateTempFile(io::Error),
    Serialize(serde_json::Error),
    Persist(io::Error),
}

impl fmt::Display for WriteJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteJsonError::DetermineDir() =>
                write!(f, "Invalid file path"),

            WriteJsonError::CreateTempFile(err) =>
                write!(f, "Failed to create temp file: {}", err),

            WriteJsonError::Serialize(err) =>
                write!(f, "Failed to serialize config: {}", err),

            WriteJsonError::Persist(err) =>
                write!(f, "Failed to persist file: {}", err),
        }
    }
}



pub enum ReadJsonError {
    Open(io::Error),
    Deserialize(serde_json::Error),
}

impl fmt::Display for ReadJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadJsonError::Open(err) =>
                write!(f, "Failed to open file: {}", err),

            ReadJsonError::Deserialize(err) =>
                write!(f, "Failed to deserialize: {}", err),
        }
    }
}

