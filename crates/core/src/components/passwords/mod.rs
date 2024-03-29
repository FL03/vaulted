/*
    Appellation: passwords <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{builder::*, password::*, utils::*};

pub(crate) mod builder;
pub(crate) mod password;

pub(crate) mod utils {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use rand::distributions::{Alphanumeric, DistString};

    pub fn generate_random_password(length: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), length)
    }

    pub fn validate_password(password: String, hash: String) -> bool {
        let hash = PasswordHash::new(hash.as_str()).expect("");
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    }
}
