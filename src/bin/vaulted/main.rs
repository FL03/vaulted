/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::core::BoxResult;
pub mod app;

#[tokio::main]
async fn main() -> BoxResult {
    let mut app = app::App::default();
    // app.run().await.expect("Failed to run the application...");
    let arch = Archive::from(".artifacts/tmp");

    println!("{:?}", arch.setup());

    Ok(())
}

pub fn read_dir_or(path: &str) -> std::fs::ReadDir {
    match std::fs::read_dir(path) {
        Ok(v) => v,
        Err(_) => {
            std::fs::create_dir_all(path).expect("");
            std::fs::read_dir(path).expect("")
        }
    }
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
        let reader = read_dir_or(self.dir.as_str());
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
