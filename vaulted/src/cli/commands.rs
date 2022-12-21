/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::*;
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Password(Passwords),
    System(System),
}

impl Commands {
    pub fn handle(&self) -> tokio::task::JoinHandle<Arc<Self>> {
        let cmds = Arc::new(self.clone());
        tokio::spawn(async move {
            cmds.handler().ok().unwrap();
            println!("{:?}", cmds.clone());
            cmds
        })
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self.clone() {
            Commands::Password(password) => {
                password.handler()?;
            }
            Commands::System(system) => {
                system.handler()?;
            }
        }
        Ok(self)
    }
}
