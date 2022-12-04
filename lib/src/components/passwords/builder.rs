/*
    Appellation: builder <passwords>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::passwords::{generate_random_password, Password};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
// use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum PasswordOptions {
    #[default]
    Alphanumeric,
    Chars,
    Num,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BuilderParams {
    pub length: usize,
    pub options: Vec<PasswordOptions>,
}

impl BuilderParams {
    pub fn new(length: usize, options: Option<Vec<PasswordOptions>>) -> Self {
        let options = options.unwrap_or(vec![Default::default()]);
        Self { length, options }
    }
}

impl Default for BuilderParams {
    fn default() -> Self {
        Self::new(12, None)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum BuilderState {
    #[default]
    Base,
    Hashing,
    Complete,
}

#[derive(Clone, Default)]
pub struct PasswordBuilder<'a> {
    argon2: Argon2<'a>,
    pub password: Password,
    pub state: BuilderState,
}

impl<'a> PasswordBuilder<'a> {
    pub fn new() -> Self {
        let argon2 = Argon2::default();
        let password = Password::default();
        let state = BuilderState::default();
        Self {
            argon2,
            password,
            state,
        }
    }
    pub fn engine(&self) -> Argon2 {
        self.argon2.clone()
    }
    pub fn generate(&mut self, length: usize) -> &Self {
        self.password = Password::new(generate_random_password(length));
        self
    }
    pub fn hash(&mut self) -> argon2::password_hash::Output {
        let salt = &self.salter();
        let res = self
            .engine()
            .hash_password(self.password.to_string().as_bytes(), salt)
            .expect("")
            .hash
            .unwrap();
        self.password = Password::new(res.to_string());
        res
    }
    pub fn salter(&self) -> SaltString {
        SaltString::generate(&mut rand::rngs::OsRng)
    }
    pub fn password(&self) -> &Password {
        &self.password
    }
    pub fn validate(&self, password: &[u8]) -> bool {
        let salt = &self.salter();
        let hash = self.engine().hash_password(password, salt);
        match hash {
            Err(_) => false,
            Ok(v) => self.verify_password(password, &v).is_ok(),
        }
    }
}

impl<'a> PasswordVerifier for PasswordBuilder<'a> {
    fn verify_password(
        &self,
        password: &[u8],
        hash: &argon2::PasswordHash<'_>,
    ) -> argon2::password_hash::Result<()> {
        Argon2::default().verify_password(password, hash)
    }
}
