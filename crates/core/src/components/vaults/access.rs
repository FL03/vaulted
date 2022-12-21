/*
    Appellation: access <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VaultAccess {
    ApiKey { public: String, secret: String },
    Passphrase(String),
    UserPass { username: String, password: String },
}

impl From<String> for VaultAccess {
    fn from(data: String) -> Self {
        Self::Passphrase(data)
    }
}

impl Default for VaultAccess {
    fn default() -> Self {
        Self::Passphrase(Default::default())
    }
}

impl std::fmt::Display for VaultAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_access_default() {
        let a = VaultAccess::default();
        let b = VaultAccess::Passphrase(String::new());
        assert_eq!(a, b)
    }
}
