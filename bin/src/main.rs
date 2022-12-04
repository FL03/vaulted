/*
    Appellation: vaulted <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{context::*, settings::*};

pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;

use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    let mut app = App::default();
    app.start(None).await?;

    Ok(())
}

pub fn setup_workdir(workdir: Option<&str>) -> BoxResult<std::path::PathBuf> {
    // If provided an option, get the path or set the working directory to the current dir and
    let mut wd = match workdir {
        Some(v) => std::path::PathBuf::from(v),
        None => std::env::current_dir().unwrap_or_default(),
    };
    // Sets the current directory to the newly created workdir
    std::env::set_current_dir(wd.clone())?;
    tracing::info!("Setting up the working directory: {:?}", &wd);
    wd.push(".vault");
    // Creates the directories needed for the application within the specified workdir
    std::fs::create_dir_all(wd.clone())?;
    tracing::info!("Success: Created new directory {} ", ".vault");
    Ok(wd)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct App {
    ctx: Context,
}

impl App {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub async fn runtime(&self) -> BoxResult<&Self> {
        let cli = cli::CommandLineInterface::new();
        tracing::info!("{:?}", cli.handler()?);

        Ok(self)
    }
    pub fn settings(&self) -> &Settings {
        &self.ctx.settings
    }
    pub async fn setup(&self, workdir: Option<&str>) -> BoxResult<&Self> {
        let mut logger = self.settings().logger.clone().unwrap_or_default();
        logger.setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Initializing the application and associated services...");

        let workdir = setup_workdir(workdir)?;
        tracing::info!("{:?}", workdir);

        let mut tmp = std::env::current_dir()?;
        tmp.push(".vault/credentials");

        std::fs::create_dir_all(tmp)?; // Attempts to create the app directory within /tmp/vaulted

        Ok(self)
    }
    /// Quickstart the application with a simple method bootstrapping implied capabilities
    pub async fn start(&mut self, workdir: Option<&str>) -> BoxResult {
        self.setup(workdir).await?;
        self.runtime().await?;

        Ok(())
    }
}

impl std::convert::From<Settings> for App {
    fn from(settings: Settings) -> Self {
        Self::new(Context::new(settings))
    }
}
