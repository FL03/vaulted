/*
    Appellation: password <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::utils::generate_random_password;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
    pub fn random(length: usize) -> Self {
        Self::new(generate_random_password(length))
    }
    pub fn argon2(&self) -> Argon2 {
        Argon2::default()
    }
    pub fn hash(&mut self) -> BoxResult<&Self> {
        let salt = SaltString::generate(&mut rand::rngs::OsRng);
        let hash = self
            .argon2()
            .hash_password(self.0.as_bytes(), &salt)
            .expect("")
            .hash
            .unwrap();
        self.0 = hash.clone().to_string();
        Ok(self)
    }
    pub fn password(&self) -> &String {
        &self.0
    }
    pub fn validate(&self, password: &[u8]) -> bool {
        let salt = SaltString::generate(&mut rand::rngs::OsRng);
        let hash = Argon2::default().hash_password(password, &salt);
        match hash {
            Err(_) => false,
            Ok(v) => Argon2::default().verify_password(password, &v).is_ok(),
        }
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<'a> From<PasswordHash<'a>> for Password {
    fn from(data: PasswordHash<'a>) -> Self {
        Self::new(data.to_string())
    }
}

impl From<&str> for Password {
    fn from(data: &str) -> Self {
        Self::new(data.to_string())
    }
}

impl<T: ToString> From<&T> for Password {
    fn from(data: &T) -> Self {
        Self::new(data.to_string())
    }
}

impl From<usize> for Password {
    fn from(data: usize) -> Self {
        Self::new(generate_random_password(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_password() {
        let a: String = Password::random(12).to_string();
        assert_eq!(a.len(), 12);
    }

    #[test]
    fn test_password() {
        let pwd: String = Password::random(12).to_string();

        let mut a = Password::new(pwd.clone());
        assert!(a.hash().is_ok());
        assert!(a.validate(pwd.as_bytes()));

        let b = Password::from("sample");
        assert_ne!(a, b);
    }

    #[test]
    fn test_valid_password() {
        let mut a = Password::from("sample");
        assert!(a.hash().is_ok());
        assert!(a.validate("sample".as_bytes()));
    }
}
