/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;
pub mod server;

use scsys::prelude::{BoxResult, Message, State};
use serde_json::json;
use std::{
    convert::From,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> BoxResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.start().await?;

    Ok(())
}

pub type Locked<T> = Arc<Mutex<T>>;


#[derive(Clone, Debug)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: Locked<State<Message>>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context, state: Locked<State<Message>>) -> Self {
        cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");
        Self { cnf, ctx, state }
    }
    // initializes a pack of channels
    pub fn channels<T>(&self, buffer: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
        tokio::sync::mpsc::channel::<T>(buffer)
    }
    /// Change the application state
    pub async fn set_state(&mut self, state: State<Message>) -> BoxResult<&Self> {
        // Update the application state
        self.state = Arc::new(Mutex::new(state.clone()));
        // Post the change of state to the according channel(s)
        self.channels(1).0.send(self.state.clone()).await?;
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        let cli = cli::new();
        self.set_state(State::from(Message::from(json!({"cli": cli.clone()}))))
            .await?;
        // Fetch the initialized cli and process the results
        cli.handle().await?;
        self.set_state(State::from(Message::from(json!({"results": ""}))))
            .await?;
        Ok(())
    }
    /// Function wrapper for returning the current application state
    pub fn state(&self) -> &Locked<State<Message>> {
        &self.state
    }
    /// AIO method for running the initialized application
    pub async fn start(&mut self) -> BoxResult<&Self> {
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::new(data.clone(), Context::from(data), Default::default())
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data.clone().cnf, data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
