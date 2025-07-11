use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("invalid path")]
    InvalidPath,
}

pub struct Library {
    pub path: PathBuf,
    pub name: String,
}

impl Library {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self { path, name }
    }

    pub fn create(&mut self) -> Result<(), LibraryError> {
        todo!("Implement library creation logic");
    }

    pub fn delete(&mut self) -> Result<(), LibraryError> {
        // remove library assets
        todo!("Implement library deletion logic");
    }

    pub fn update(&mut self, path: Option<PathBuf>, name: Option<String>) {
        if let Some(p) = path {
            self.path = p;
        }
        if let Some(n) = name {
            self.name = n;
        }
    }
}

#[cfg(test)]
#[path = "./library.tests.rs"]
mod tests;
