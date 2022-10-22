/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{context::*, interface::*, settings::*};

pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    println!("{:?}",  std::env::current_dir());
    println!("{:?}",  std::env::temp_dir());
    let app = App::default();
    app.setup(None)?.run().await?;
    println!("{:?}",  std::env::current_dir());
    Ok(())
}

pub(crate) mod interface {
    use super::{
        cli::{cmds::Commands, CommandLineInterface},
        Context, Settings,
    };
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    pub struct App {
        ctx: Context,
    }

    impl App {
        pub fn new(ctx: Context) -> Self {
            Self { ctx }
        }
        pub fn settings(&self) -> &Settings {
            &self.ctx.settings
        }
        /* 
            During setup the application automatically creates a temporary directory and reframes the workdir
        */
        pub fn setup(&self, tmp: Option<&str>) -> BoxResult<&Self> {
            tracing_subscriber::fmt::init();

            let mut tmp = match tmp {
                Some(v) => std::path::PathBuf::from(v),
                None => std::env::temp_dir()
            };
            tracing::info!("{:?}", tmp);
            tmp.push("vaulted");
            
            std::fs::create_dir_all(tmp.clone())?; // Attempts to create the app directory within /tmp/vaulted
            std::env::set_current_dir(tmp)?; // Attempts to set the working directory to the specified path

            Ok(self)
        }
        pub async fn run(&self) -> BoxResult<&Self> {
            let cli = CommandLineInterface::new();
            tracing::info!("{:?}", cli.handler()?);

            Ok(self)
        }
    }

    impl std::convert::From<Settings> for App {
        fn from(settings: Settings) -> Self {
            Self::new(Context::new(settings))
        }
    }
}
