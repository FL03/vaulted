
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VaultAccess {
    ApiKey {
        api_key: String,
    },
    Std {
        username: String,
        password: String
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VaultMetadata {
    pub accessed: Timestamp,
    pub created: Timestamp
}

impl VaultMetadata {
    pub fn new() -> Self {
        let accessed = Timestamp::default();
        let created = Timestamp::default();
        Self { accessed, created }
    }
}

impl Default for VaultMetadata {
    fn default() -> Self {
        Self::new()
    }
}