//! This module provides functionality for reading and writing application and configuration files.
//!
//! It defines structs for `AppConfigFile` and `DeviceAuthBinary` and provides methods to interact with these files.


use std::error::Error as StdError;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use directories::BaseDirs;
use serde::de::DeserializeOwned;
use colored::*;

/// Represents the application configuration file.
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfigFile {
    /// The access token.
    pub access_token: String,
    /// The refresh token.
    pub refresh_token: String,
    /// The expiration time in seconds.
    pub expires_in: isize,
    /// The list of scopes.
    pub scope: Vec<String>,
}


/// Represents the device authentication binary file.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceAuthBinary {
    /// The device code.
    pub device_code: String,
    /// The expiration time in seconds.
    pub expires_in: i32,
    /// The interval in seconds.
    pub interval: i64,
    /// The user code.
    pub user_code: String,
    /// The verification URI.
    pub verification_uri: String,
    /// Indicates whether the device has been verified.
    pub has_verified: bool,
}

impl DeviceAuthBinary {
    /// Creates a new instance of `DeviceAuthBinary`.
    ///
    /// # Arguments
    ///
    /// * `dc` - The device code.
    /// * `exp` - The expiration time in seconds.
    /// * `int` - The interval in seconds.
    /// * `uc` - The user code.
    /// * `vu` - The verification URI.
    /// * `veri` - Indicates whether the device has been verified.
    pub fn new(dc: String, exp: i32, int: i64, uc: String, vu: String, veri: bool) -> DeviceAuthBinary {
        DeviceAuthBinary {
            device_code: dc,
            expires_in: exp,
            interval: int,
            user_code: uc,
            verification_uri: vu,
            has_verified: veri,
        }
    }
}

impl AppConfigFile {
/// Creates a new instance of `AppConfigFile`.
///
/// # Arguments
///
/// * `at` - The access token.
/// * `rt` - The refresh token.
/// * `exp` - The expiration time in seconds.
/// * `scope` - The list of scopes.
    pub fn new(at: String, rt: String, exp: isize, scope: Vec<String>) -> AppConfigFile {
        AppConfigFile {
            access_token: at,
            refresh_token: rt,
            expires_in: exp,
            scope,
        }
    }
}


pub enum FileCategory {
    App,
    Config
}

impl FileCategory {
    pub fn as_str(&self) -> &str {
        match self {
            FileCategory::App => "app",
            FileCategory::Config => "config",
        }
    }
}


/// Writes the given data to a file.
///
/// # Arguments
///
/// * `data` - The data to be written to the file.
/// * `file_name` - The name of the file.
/// * `file_type` - The category of the file.
///
/// # Errors
///
/// Returns an error if the file writing fails.

pub fn write_to_file<T: Serialize>(data: &T, file_name: &str, file_type: FileCategory) -> Result<(), Box<dyn StdError>> {
    println!("{}", "Writing to file".green());
    let file_path = get_file_path(file_name, file_type.as_str())?;
    ensure_directory_exists(&file_path)?;
    let serialized_data = bincode::serialize(data)?;
    println!("{}: {}", "File written to".green(), file_path.to_str().unwrap_or_default());
    fs::write(file_path, serialized_data)?;
    Ok(())
}


/// Reads data from a file.
///
/// # Arguments
///
/// * `file_name` - The name of the file.
/// * `file_type` - The category of the file.
///
/// # Errors
///
/// Returns an error if the file reading fails.
pub fn read_from_file<T: DeserializeOwned>(file_name: &str, file_type: FileCategory) -> Result<T, Box<dyn StdError>> {
    let file_path = get_file_path(file_name, file_type.as_str())?;
    let serialized_data = fs::read(file_path)?;
    let data: T = bincode::deserialize(&serialized_data)?;
    Ok(data)
}


/// Updates the file with the given data.
///
/// # Arguments
///
/// * `data` - The data to be written to the file.
/// * `file_name` - The name of the file.
/// * `file_type` - The category of the file.
///
/// # Errors
///
/// Returns an error if the file updating fails.
pub fn update_file<T: Serialize>(data: &T, file_name: &str, file_type: FileCategory) -> Result<(), Box<dyn StdError>> {
    write_to_file(data, file_name, file_type)
}

/// Checks if a file exists.
///
/// # Arguments
///
/// * `file_name` - The name of the file.
/// * `file_type` - The category of the file.
///
/// # Returns
///
/// `true` if the file exists, `false` otherwise.
pub fn file_exists(file_name: &str, file_type: &str) -> bool {
    let file_path = get_file_path(file_name, file_type).unwrap_or_default();
    file_path.exists()
}

/// Gets the file path for the given file name and type.
///
/// # Arguments
///
/// * `file_name` - The name of the file.
/// * `file_type` - The category of the file.
///
/// # Errors
///
/// Returns an error if the file path cannot be determined.
pub fn get_file_path(file_name: &str, file_type: &str) -> Result<PathBuf, Box<dyn StdError>> {
    let base_dirs = BaseDirs::new().ok_or("Failed to determine base directories")?;
    let data_dir = base_dirs.data_dir();
    let file_path = data_dir.join(file_type).join(file_name);
    Ok(file_path)
}

/// Ensures that the directory for the given file path exists.
///
/// # Arguments
///
/// * `file_path` - The path of the file.
///
/// # Errors
///
/// Returns an error if the directory creation fails.
pub fn ensure_directory_exists(file_path: &Path) -> Result<(), Box<dyn StdError>> {
    let directory = file_path.parent().ok_or("Invalid file path")?;
    fs::create_dir_all(directory)?;
    Ok(())
}