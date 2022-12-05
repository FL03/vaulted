/*
    Appellation: vault <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Vaults are secure storage solutions for credentials managed by the application
*/
use super::VaultAccess;
use crate::{models::Credential, read_files_in_dir, to_json};
use scsys::prelude::bson::oid::ObjectId;
use scsys::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Vault {
    access: VaultAccess,
    workdir: String,
    data: Vec<VaultMetadata>,
}

impl Vault {
    pub fn new(access: VaultAccess, workdir: String, data: Vec<VaultMetadata>) -> Self {
        Self {
            access,
            workdir,
            data,
        }
    }
    pub fn export(&self, save_as: Option<&str>) -> std::io::Result<&Self> {
        let fname = save_as.unwrap_or("vault");
        let path = format!("{}/{}.json", self.workdir, fname);

        to_json(path.as_str(), json!(self.data))?;
        Ok(self)
    }
    pub fn paths(&self) -> std::io::Result<Vec<PathBuf>> {
        read_files_in_dir(self.workdir.as_str())
    }
    // Snapshot the vault, saving the data to a .json file with defaults set to "vault.json"
    pub fn snapshot(&mut self, save_as: Option<&str>) -> std::io::Result<&Self> {
        std::fs::write(
            save_as.unwrap_or("vault.json"),
            serde_json::to_string_pretty(&self).unwrap(),
        )?;

        Ok(self)
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new(
            VaultAccess::default(),
            crate::TMP_DIR.to_string(),
            Vec::new(),
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VaultMetadata {
    pub id: ObjectId,
    pub credentials: Option<Vec<Credential>>,
    pub data: Vec<Self>,
    pub homepage: Option<String>,
    pub links: Option<Vec<String>>,

    pub timestamps: Vec<Timestamp>,
}

impl VaultMetadata {
    pub fn new(
        credentials: Option<Vec<Credential>>,
        data: Vec<Self>,
        homepage: Option<String>,
        links: Option<Vec<String>>,
    ) -> Self {
        let id = ObjectId::new();
        let timestamps = vec![Timestamp::default()];
        Self {
            id,
            credentials,
            data,
            homepage,
            links,
            timestamps,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        // Create a new vault instance
        let mut a = Vault::default();
        // Snapshot the vault, saving the data to a .json file with defaults set to
        assert!(a.snapshot(None).is_ok());
        //
        let mut path = std::env::current_dir().ok().unwrap();
        path.push("vault.json");
        if path.exists() {
            assert!(true);
            std::fs::remove_file(path.clone()).unwrap();
        } else {
            assert!(false)
        }
    }
}
