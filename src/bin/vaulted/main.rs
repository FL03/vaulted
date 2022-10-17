/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::app::*;

pub(crate) mod app;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    let mut app = App::default();
    app.with_logging();
    app.run().await?;

    Ok(())
}
