/*
    Appellation: credential <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::{Id, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Credential {
    pub id: Id,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub timestamp: Timestamp,
}

impl Credential {
    pub fn new(key: String, label: Option<String>) -> Self {
        let id = Id::default();
        let timestamp = Timestamp::default();
        Self {
            id,
            key,
            label,
            timestamp,
        }
    }
}

impl std::convert::From<&Self> for Credential {
    fn from(data: &Self) -> Self {
        data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Credential::default();
        let b = a.clone();
        assert_eq!(&a, &b)
    }
}
