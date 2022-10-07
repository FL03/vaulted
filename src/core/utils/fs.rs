/*
    Appellation: fs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde_json::Value;
use std::{fs::{ReadDir, create_dir_all, read_dir, read_to_string}, path::PathBuf};

pub fn create_json_file(path: &str, data: Value) -> std::io::Result<()> {
    std::fs::write(path,serde_json::to_string_pretty(&data).unwrap())
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

pub fn read_json_file(path: &str) -> Value {
    serde_json::from_str::<Value>(&read_to_string(path).unwrap()).unwrap()
}
