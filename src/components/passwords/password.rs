/*
    Appellation: password <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::utils::generate_random_password;
use argon2::{password_hash::{rand_core::OsRng,PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Password(pub String);

impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
    pub fn generate(length: usize) -> Self {
        Self::new(generate_random_password(length))
    }
    pub fn hasher(&self) -> Argon2 {
        Argon2::default()
    }
    pub fn hash_password(&mut self) -> &Self {
        let salt = self.salt();
        let hashed = self.hasher().hash_password(self.0.as_bytes(), &salt).expect("");
        self.0 = hashed.to_string();
        self
    }
    pub fn salt(&self) -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PasswordBuilder {
    pub password: String,
    pub salt: Option<String>
}

impl PasswordBuilder {
    pub fn new(password: String, salt: Option<String>) -> Self {
        Self { password, salt }
    }
    pub fn generate_password(&mut self, length: usize) -> &Self {
        self.password = generate_random_password(length);
        self
    }
    pub fn hasher(&self) -> Argon2 {
        Argon2::default()
    }
    pub fn hash_password(&mut self) -> BoxResult<&Self> {
        let password = self.password.as_bytes();
        self.password = self.hasher().hash_password(password, &self.salt()).unwrap().to_string();
        Ok(self)
    }
    pub fn salt(&self) -> SaltString {
        SaltString::generate(&mut OsRng)
    }
    pub fn size(&self) -> usize {
        self.password.len()
    }
    pub fn validate(&self, parsed: PasswordHash) -> bool {
        let password = self.password.as_bytes();
        self.hasher().verify_password(&password, &parsed).is_ok()
    }
}
