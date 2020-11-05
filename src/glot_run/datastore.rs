use std::collections::HashMap;
use std::path::Path;
use std::fmt;

use crate::glot_run::file;



type DataStore<E> = HashMap<String, E>;


pub fn init<E: serde::Serialize>(path: &Path) -> Result<(), file::WriteJsonError> {
    if !path.exists() {
        let entries: DataStore<E> = HashMap::new();
        file::write_json(path, &entries)?;
    }

    Ok(())
}

pub enum GetError {
    Read(file::ReadJsonError),
    NotFound(),
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GetError::Read(err) => {
                write!(f, "Failed to read from datastore: {}", err)
            }

            GetError::NotFound() => {
                write!(f, "Entry not found in datastore")
            }
        }
    }
}

pub fn get_entry<E>(path: &Path, key: &str) -> Result<E, GetError>
    where
        E: Clone,
        E: serde::de::DeserializeOwned {

    let entries: DataStore<E> = file::read_json(path)
        .map_err(GetError::Read)?;

    let entry = entries.get(key)
        .ok_or(GetError::NotFound())?;

    Ok(entry.clone())
}

pub fn list_values<E>(path: &Path) -> Result<Vec<E>, file::ReadJsonError>
    where
        E: Clone,
        E: serde::de::DeserializeOwned {

    let entries: DataStore<E> = file::read_json(path)?;

    let values = entries.values()
        .cloned()
        .collect();

    Ok(values)
}

pub fn find_value<F, E>(path: &Path, f: F) -> Result<E, GetError>
    where
        E: Clone,
        E: serde::de::DeserializeOwned,
        F: Copy,
        F: FnOnce(&E) -> bool {

    let entries: DataStore<E> = file::read_json(path)
        .map_err(GetError::Read)?;

    let entry = entries.iter().find_map(|(_, val)| {
        if f(&val) {
            Some(val)
        } else {
            None
        }
    }).ok_or(GetError::NotFound())?;

    Ok(entry.clone())
}


pub enum AddError {
    Read(file::ReadJsonError),
    Write(file::WriteJsonError),
}

impl fmt::Display for AddError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddError::Read(err) => {
                write!(f, "Failed to read from datastore: {}", err)
            }

            AddError::Write(err) => {
                write!(f, "Failed to write to datastore: {}", err)
            }
        }
    }
}


pub fn add_entry<E>(path: &Path, key: &str, entry: &E) -> Result<(), AddError>
    where
        E: Clone,
        E: serde::Serialize,
        E: serde::de::DeserializeOwned {

    let mut entries: DataStore<E> = file::read_json(path)
        .map_err(AddError::Read)?;

    entries.insert(key.to_string(), entry.clone());

    file::write_json(path, &entries)
        .map_err(AddError::Write)
}

pub enum UpdateError {
    Read(file::ReadJsonError),
    NotFound(),
    Write(file::WriteJsonError),
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateError::Read(err) => {
                write!(f, "Failed to read from datastore: {}", err)
            }

            UpdateError::NotFound() => {
                write!(f, "Entry not found in datastore")
            }

            UpdateError::Write(err) => {
                write!(f, "Failed to write to datastore: {}", err)
            }
        }
    }
}


pub fn update_entry<F, E>(path: &Path, key: &str, update_fn: F) -> Result<E, UpdateError>
    where
        E: Clone,
        E: serde::Serialize,
        E: serde::de::DeserializeOwned,
        F: FnOnce(&E) -> E {

    let mut entries: DataStore<E> = file::read_json(path)
        .map_err(UpdateError::Read)?;

    let old_entry = entries.get(key).ok_or(UpdateError::NotFound())?;
    let new_entry = update_fn(&old_entry);

    entries.insert(key.to_string(), new_entry.clone());

    file::write_json(path, &entries)
        .map_err(UpdateError::Write)?;

    Ok(new_entry)
}

pub fn remove_entry<E>(path: &Path, key: &str) -> Result<(), AddError>
    where
        E: serde::Serialize,
        E: serde::de::DeserializeOwned {

    let mut entries: DataStore<E> = file::read_json(path)
        .map_err(AddError::Read)?;

    entries.remove(key);

    file::write_json(path, &entries)
        .map_err(AddError::Write)
}
