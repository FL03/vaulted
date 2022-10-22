/*
    Appellation: credentials <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::{prelude::bson::oid::ObjectId, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CredentialMetadata {
    Tokens(Vec<Value>),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Credential {
    pub id: ObjectId,
}

pub trait CredentialSpec {
    fn credential(&self) -> &Self;
    fn message(&self) -> String;
    fn metadata(&self) -> Vec<Value>;
    fn timestamp(&self) -> Timestamp;
}
