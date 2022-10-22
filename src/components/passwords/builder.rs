/*
    Appellation: builder <passwords>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::password::Password;
use argon2::{Argon2, PasswordHash, SaltString};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PasswordBuilder {
    pub password: Password,
}

impl PasswordBuilder {
    pub fn new(password: Password) -> Self {
        Self { password }
    }
}
