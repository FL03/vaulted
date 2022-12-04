/*
    Appellation: archive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{artifacts::Artifact, create_json_file, read_files_in_dir};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Archive {
    pub dir: String,
    pub data: Vec<Artifact>,
}

impl Archive {
    pub fn new(dir: String, data: Vec<Artifact>) -> Self {
        Self { dir, data }
    }
    pub fn setup(dir: String) -> Self {
        let data = Vec::new();
        Self::new(dir, data)
    }
    pub fn save(&self, save_as: Option<&str>) -> std::io::Result<&Self> {
        let fname = save_as.unwrap_or("vault");
        let path = format!("{}/{}.json", self.dir, fname);
        create_json_file(path.as_str(), json!(self.data))?;
        Ok(self)
    }
    pub fn contents(&self) -> BoxResult<&Self> {
        let res = read_files_in_dir(self.dir.as_str())?;
        println!("{:?}", res);
        Ok(self)
    }
}

impl std::convert::From<&str> for Archive {
    fn from(dir: &str) -> Self {
        Self::setup(dir.to_string())
    }
}

impl Default for Archive {
    fn default() -> Self {
        Self::from("/tmp/credentials")
    }
}

pub(crate) mod utils {}
