/*
    Appellation: password <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::utils::generate_random_password;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
    pub fn argon2(&self) -> Argon2 {
        Argon2::default()
    }
    pub fn generate(&mut self, length: usize) -> &Self {
        self.0 = generate_random_password(length);
        self
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

impl std::convert::From<usize> for Password {
    fn from(data: usize) -> Self {
        Self::new(generate_random_password(data))
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::passwords::{validate_password, Password};

    #[test]
    fn test_compiles() {
        let f = |x: usize| x.pow(2);
        let a = f(2);
        let b = 4;
        assert_eq!(a, b)
    }

    #[test]
    fn test_password() {
        let a: String = Password::from(12).to_string();
        assert_eq!(a.len(), 12);

        let mut a_prime = Password::new(a.clone());
        assert!(a_prime.hash().is_ok());

        let a_hash: String = a_prime.clone().to_string();
        assert!(validate_password(a, a_hash));

        let sample_password = "sample".to_string();
        let mut b = Password::new(sample_password.clone());
        assert!(b.hash().is_ok());
        assert!(validate_password(sample_password, b.password().clone()));

        assert_ne!(a_prime, b);
    }

    #[test]
    fn test_valid_password() {
        let sample_password = "sample".to_string();
        let mut a = Password::new(sample_password.clone());
        assert!(a.hash().is_ok());
        assert!(validate_password(sample_password, a.password().clone()));
    }
}
