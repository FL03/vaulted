/*
    Appellation: vault <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::VaultAccess;
use scsys::{core::Timestamp, prelude::bson::oid::ObjectId};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Vault {
    access: VaultAccess,
    workdir: String,
    data: Vec<VaultMetadata>
}

impl Vault {
    pub fn new(access: VaultAccess, workdir: String, data: Vec<VaultMetadata>) -> Self {
        Self { access, workdir, data }
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new(VaultAccess::default(), crate::TMP_DIR.to_string(), Vec::new())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VaultMetadata {
    pub id: ObjectId,
    pub accessed: Timestamp,
    pub created: Timestamp
}

impl VaultMetadata {
    pub fn new() -> Self {
        let accessed = Timestamp::default();
        let created = Timestamp::default();
        let id = ObjectId::new();
        Self { accessed, created, id }
    }
}

impl Default for VaultMetadata {
    fn default() -> Self {
        Self::new()
    }
}
