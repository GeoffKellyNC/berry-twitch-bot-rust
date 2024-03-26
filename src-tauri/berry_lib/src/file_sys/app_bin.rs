use std::error::Error as StdError;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use directories::BaseDirs;
use serde::de::DeserializeOwned;
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfigFile {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: isize,
    pub scope: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceAuthBinary {
    pub device_code: String,
    pub expires_in: i32,
    pub interval: i64,
    pub user_code: String,
    pub verification_uri: String,
    pub has_verified: bool,
}

impl DeviceAuthBinary {
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
    pub fn new(at: String, rt: String, exp: isize, scope: Vec<String>) -> AppConfigFile {
        AppConfigFile {
            access_token: at,
            refresh_token: rt,
            expires_in: exp,
            scope,
        }
    }
}

pub fn write_to_file<T: Serialize>(data: &T, file_name: &str, file_type: &str) -> Result<(), Box<dyn StdError>> {
    println!("{}", "Writing to file".green());
    let file_path = get_file_path(file_name, file_type)?;
    ensure_directory_exists(&file_path)?;
    let serialized_data = bincode::serialize(data)?;
    println!("{}: {}", "File written to".green(), file_path.to_str().unwrap_or_default());
    fs::write(file_path, serialized_data)?;
    Ok(())
}

pub fn read_from_file<T: DeserializeOwned>(file_name: &str, file_type: &str) -> Result<T, Box<dyn StdError>> {
    let file_path = get_file_path(file_name, file_type)?;
    let serialized_data = fs::read(file_path)?;
    let data: T = bincode::deserialize(&serialized_data)?;
    Ok(data)
}

pub fn update_file<T: Serialize>(data: &T, file_name: &str, file_type: &str) -> Result<(), Box<dyn StdError>> {
    write_to_file(data, file_name, file_type)
}

pub fn file_exists(file_name: &str, file_type: &str) -> bool {
    let file_path = get_file_path(file_name, file_type).unwrap_or_default();
    file_path.exists()
}

pub fn get_file_path(file_name: &str, file_type: &str) -> Result<PathBuf, Box<dyn StdError>> {
    let base_dirs = BaseDirs::new().ok_or("Failed to determine base directories")?;
    let data_dir = base_dirs.data_dir();
    let file_path = data_dir.join(file_type).join(file_name);
    Ok(file_path)
}

pub fn ensure_directory_exists(file_path: &Path) -> Result<(), Box<dyn StdError>> {
    let directory = file_path.parent().ok_or("Invalid file path")?;
    fs::create_dir_all(directory)?;
    Ok(())
}