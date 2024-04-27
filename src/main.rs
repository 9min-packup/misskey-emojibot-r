use dotenv::dotenv;
use util::logger::init_logger;

mod util;
mod misskey_api;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use util::env::get_string_env;
#[cfg(debug_assertions)]
use debug::debug_misskey_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_logger();
    // Debug
    #[cfg(debug_assertions)]
    {
        debug::debug_toml();

        let debug_mi= get_string_env("DEBUG_MI", "no");
        match debug_mi.as_str() {
            "yes" => debug_misskey_api().await?,
            _ => (),
        }
    }

    Ok(())
}