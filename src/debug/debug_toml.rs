use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use log::debug;
use crate::util::toml::{read_toml, write_toml};
use crate::util::env::get_string_env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    bot : BotConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bot : BotConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    host : String,
    token : String,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            host: String::from("example.tld"),
            token: String::from("XXXXXXXXXXXXXXXX"),
        }
    }
}

pub fn debug_toml() {
    let config = debug_read_config();
    debug_write_config(config);
}

pub fn debug_read_config() -> Config {
    let path : String = get_string_env("DEBUG_TOML_READ_FILE", "./config.toml");

    let config = read_toml::<Config>(&path).unwrap();
    debug!("{:?}", config);

    config
}

pub fn debug_write_config(config : Config) {
    let path : String = get_string_env("DEBUG_TOML_WRITE_FILE", "./store.toml");

    write_toml(&path, config).unwrap();
}