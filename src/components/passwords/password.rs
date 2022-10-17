/*
    Appellation: password <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Deserialize, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum PasswordChar {
    #[default]
    All,
    AlphabetAll,
    AlphabetUpper,
    AlphabetLower,
    Charecters,
    Numbers,
    SpecialCharecters,
}

#[derive(Clone, Debug, Deserialize, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PasswordParams {
    pub length: Option<usize>,
    pub usechars: Option<Vec<PasswordChar>>,
}

#[derive(Clone, Debug, Deserialize, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Password(pub String);

impl Password {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn generate(&mut self, params: Option<PasswordParams>) -> &Self {
        let params = params.unwrap_or_default();
        self
    }
}
