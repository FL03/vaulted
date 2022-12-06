/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::args::CRUDArgs;
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use vaulted::passwords::PasswordBuilder;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Password {
        #[clap(value_enum)]
        action: CRUDArgs,
        #[clap(long, short, value_parser)]
        length: Option<usize>,
    },
    Vault {
        #[clap(long, short, value_parser)]
        query: String,
    },
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        match self {
            Self::Password { action, length } => {
                // let length = length.unwrap_or_default();
                match *action {
                    CRUDArgs::Create => {
                        let length = match length {
                            Some(v) => *v,
                            None => 12,
                        };
                        let mut builder = PasswordBuilder::new();
                        tracing::debug!("Process: Generating a new password...");
                        builder.generate(length);

                        let password = builder.password().clone();
                        println!("Created a new password: {}", password);
                    }
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
            Self::Vault { query } => {
                let _query = query.clone();
            }
        };
        Ok(self)
    }
}

pub async fn handle_crud<S, T>(
    action: &CRUDArgs,
    _transition: &dyn Fn(S) -> BoxResult<T>,
) -> BoxResult {
    match *action {
        CRUDArgs::Create => {}
        CRUDArgs::Read => {}
        CRUDArgs::Update => {}
        CRUDArgs::Delete => {}
    };
    Ok(())
}

pub fn transitioner<S, T>(ctx: S, actor: &dyn Fn(S) -> T) -> T {
    actor(ctx)
}

pub fn app_mode_handler(mode: String) -> BoxResult {
    println!("{:?}", mode);

    Ok(())
}
