/*
    Appellation: password <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::utils::generate_random_password;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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
        self.0 = self
            .hasher()
            .hash_password(self.0.as_bytes(), &salt)
            .expect("")
            .to_string();
        self
    }
    pub fn salt(&self) -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

// impl std::convert::From<&Self> for Password {
//     fn from(data: &Self) -> Self {
//         Self::new(data.0)
//     }
// }

impl<'a> std::convert::From<PasswordHash<'a>> for Password {
    fn from(data: PasswordHash<'a>) -> Self {
        Self::new(data.to_string())
    }
}

impl<T: std::string::ToString> std::convert::From<&T> for Password {
    fn from(data: &T) -> Self {
        Self::new(data.to_string())
    }
}

impl std::convert::Into<String> for Password {
    fn into(self) -> String {
        self.0.clone()
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
