/*
    Appellation: accounts <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::{prelude::bson::oid::ObjectId, Timestamp};
use serde::{Deserialize, Serialize};

pub trait AccountSpec {
    fn account_id(&self) -> ObjectId;
    fn homepage(&self) -> String;
    fn metadata(&self) -> Vec<AccountMetadata>;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
    fn timestamp(&self) -> Timestamp {
        Timestamp::default()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Account {
    pub id: ObjectId,

    pub homepage: String,
    pub name: String,
    pub slug: String,
    pub data: Vec<AccountMetadata>,
}

impl Account {
    pub fn new(homepage: String, name: String) -> Self {
        let id = ObjectId::new();
        let slug = name.to_lowercase();
        let data = Vec::new();
        Self {
            id,
            homepage,
            name,
            slug,
            data,
        }
    }
    pub fn from_str(homepage: &str, name: &str) -> Self {
        Self::new(homepage.to_string(), name.to_string())
    }
    pub fn slug(&self) -> String {
        self.name.to_lowercase()
    }
}

impl AccountSpec for Account {
    fn account_id(&self) -> ObjectId {
        self.id
    }

    fn homepage(&self) -> String {
        self.homepage.clone()
    }

    fn metadata(&self) -> Vec<AccountMetadata> {
        self.data.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AccountMetadata {
    Empty,
    Tokens {
        key: Option<String>,
        passphrase: Option<String>,
    },
}

impl Default for AccountMetadata {
    fn default() -> Self {
        Self::Empty
    }
}
