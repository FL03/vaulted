/*
    Appellation: models <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{accounts::*, credentials::*};

mod accounts;
mod credentials;

pub trait SecureModel {
    fn timestamp(&self);
}
