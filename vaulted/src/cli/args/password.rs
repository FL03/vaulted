/*
    Appellation: builder <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use vaulted_sdk::prelude::PasswordBuilder;
use clap::{Args, ValueEnum};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
pub enum Crud {
    #[default]
    Create = 0,
    Read = 1,
    Update = 2,
    Delete = 3
}


#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Passwords {
    #[clap(value_enum)]
    action: Option<Crud>,
    #[clap(default_value = None, long, short, value_parser)]
    length: Option<usize>
}

impl Passwords {
    pub fn new(action: Option<Crud>, length: Option<usize>) -> Self {
        Self { action, length }
    }
    fn commands(&self) -> BoxResult<&Self> {
        if let Some(action) = self.action.clone() {
            match action as i64 {
                0 => {
                    let mut builder = PasswordBuilder::new();
                    tracing::debug!("Process: Generating a new password...");
                    builder.generate(self.length.unwrap_or(12));
    
                    let password = builder.password().clone();
                    println!("Created a new password: {}", password);
                }
                1 => {}
                2 => {}
                _ => {}
            };
        }
        
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::debug!("Connector Initialized; handling inputs...");
        self.commands()?;
        Ok(self)
    }
}
