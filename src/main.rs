use dotenv::dotenv;
use log::{trace, debug, info, warn, error};
mod init_logger;
mod misskey_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_logger::init();
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
    misskey_api::hoge2().await?;
    Ok(())
}