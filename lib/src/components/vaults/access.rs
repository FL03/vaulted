/*
    Appellation: access <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VaultAccess {
    ApiKey { api_key: String },
    None,
    Std { username: String, password: String },
}

impl Default for VaultAccess {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_access_default() {
        let a = VaultAccess::default();
        let b = VaultAccess::None;
        assert_eq!(a, b)
    }
}
