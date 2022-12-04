/*
    Appellation: args <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

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
    PartialEq,
    Serialize,
    ValueEnum,
)]
#[strum(serialize_all = "snake_case")]
pub enum Runtime {
    On,
    Off,
    #[default]
    Idle,
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
    PartialEq,
    Serialize,
    ValueEnum,
)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDArgs {
    #[default]
    Create,
    Read,
    Update,
    Delete,
}

impl CRUDArgs {
    pub fn create() -> Self {
        Self::Create
    }
    pub fn read() -> Self {
        Self::Read
    }
    pub fn update() -> Self {
        Self::Update
    }
    pub fn delete() -> Self {
        Self::Delete
    }
}

pub fn catalyst<S, T>(f: &dyn Fn(S) -> T, data: S) -> scsys::BoxResult<T> {
    let res = f(data);
    Ok(res)
}
