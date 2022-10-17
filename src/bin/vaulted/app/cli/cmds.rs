/*
    Appellation: cmds <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::args::CRUDArgs;
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    App {
        #[clap(long, short, value_parser)]
        mode: Option<String>,
    },
    Vault {
        #[clap(value_enum)]
        action: CRUDArgs,
        #[clap(long, short, value_parser)]
        identifier: String,
    },
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        match self {
            Self::App { mode } => {}
            Self::Vault { action, identifier } => {
                let action = match action.clone() {
                    CRUDArgs::Create => {}
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
        };
        Ok(self)
    }
}

pub fn app_mode_handler(mode: String) -> BoxResult {
    println!("{:?}", mode);

    Ok(())
}
