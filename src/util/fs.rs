use rlua::{Error, Result};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Utility to read a file to a [`String`] from a path
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path).map_err(|e| Error::RuntimeError(e.to_string()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| Error::RuntimeError(e.to_string()))?;
    Ok(contents)
}
