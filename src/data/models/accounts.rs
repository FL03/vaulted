/*
    Appellation: accounts <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::prelude::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Account {
    pub id: ObjectId,
    pub key: String,

    pub homepage: String,
    pub data: Vec<AccountMetadata>
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AccountMetadata {
    Empty,
    Tokens {
        key: Option<String>,
        passphrase: Option<String>
    },
}

impl Default for AccountMetadata {
    fn default() -> Self {
        Self::Empty
    }
}
