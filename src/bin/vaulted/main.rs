/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::core::BoxResult;
pub mod cli;
pub mod app;

#[tokio::main]
async fn main() -> BoxResult {
    let mut app = App::default();
    app.run().await.expect("Failed to run the application...");
    let arch = Archive::from(".artifacts/tmp");

    println!("{:?}", arch.setup());

    Ok(())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Archive {
    pub dir: String,
    pub data: Option<Vec<String>>,
}

impl Archive {
    pub fn new(dir: String) -> Self {
        let data = Some(Vec::new());

        Self { dir, data }
    }
    pub fn setup(&self) -> &Self {
        let reader = match std::fs::read_dir(self.dir.clone()) {
            Ok(v) => v,
            Err(_) => {
                std::fs::create_dir_all(self.dir.clone()).expect("");
                std::fs::read_dir(self.dir.clone()).expect("")
            }
        };
        println!("{:?}", reader);
        self
    }
}

impl std::convert::From<&str> for Archive {
    fn from(dir: &str) -> Self {
        Self::new(dir.to_string())
    }
}

impl Default for Archive {
    fn default() -> Self {
        Self::from("/tmp/credentials")
    }
}
