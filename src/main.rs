use dotenv::dotenv;
use log::{debug};
mod init_logger;
mod misskey_api;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_logger::init();
    misskey_api::demo::demo().await?;
    Ok(())
}