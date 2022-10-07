/*
    Appellation: access <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VaultAccess {
    ApiKey {
        api_key: String,
    },
    None,
    Std {
        username: String,
        password: String
    }
}

impl Default for VaultAccess {
    fn default() -> Self {
        Self::None
    }
}
