/*
    Appellation: fs <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde_json::Value;
use std::fs::{create_dir_all, read_dir, read_to_string, ReadDir};
use std::path::PathBuf;

pub fn to_json(path: &str, data: Value) -> std::io::Result<()> {
    std::fs::write(path, serde_json::to_string_pretty(&data)?)
}

pub fn read_dir_or(path: &str) -> ReadDir {
    match read_dir(path) {
        Ok(v) => v,
        Err(_) => {
            create_dir_all(path).expect("");
            read_dir(path).expect("")
        }
    }
}

pub fn read_files_in_dir(path: &str) -> std::io::Result<Vec<PathBuf>> {
    let dir = read_dir(path)?;
    let res = dir
        .map(|i| i.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    Ok(res)
}

pub fn from_json(path: &str) -> Value {
    serde_json::from_str::<Value>(&read_to_string(path).unwrap()).unwrap()
}
