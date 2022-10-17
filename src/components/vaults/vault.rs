/*
    Appellation: vault <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::VaultAccess;
use crate::models::Credential;

use scsys::{prelude::bson::oid::ObjectId, Timestamp};
use serde::{Deserialize, Serialize};

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
