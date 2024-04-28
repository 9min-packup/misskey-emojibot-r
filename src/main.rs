use dotenv::dotenv;
use util::logger::init_logger;
use log::debug;

mod util;
mod misskey_api;
mod emojibot;

#[cfg(debug_assertions)]
use util::env::get_string_env;
#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use debug::debug_misskey_api;
#[cfg(debug_assertions)]
use debug::debug_toml;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_logger();

    // debug
    #[cfg(debug_assertions)]
    {
        match get_string_env("DEBUG_MI", "no").as_str() {
            "yes" => debug_misskey_api().await?,
            _ => (),
        }
        match get_string_env("DEBUG_TOML", "no").as_str() {
            "yes" => debug_toml(),
            _ => (),
        }

        match get_string_env("DEBUG_DO_NOT_RUN", "no").as_str() {
            "yes" => {
                debug!("do not run");
                return Ok(());
            },
            _ => (),
        }

    }

    emojibot::run().await?;

    Ok(())
}