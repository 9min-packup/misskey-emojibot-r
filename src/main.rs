use dotenv::dotenv;
mod util;
mod misskey_api;
mod debug;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    util::init_logger();

    // Debug
    let debug_mi= util::get_string_env("DEBUG_MI", "no");
    match debug_mi.as_str() {
        "yes" => debug::debug_misskey_api().await?,
        _ => (),
    }

    Ok(())
}