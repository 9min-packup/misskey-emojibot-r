use std::{env, fmt::Debug};
use serde::{Deserialize, Serialize};
use log::debug;
use crate::util::toml::{read_toml, write_toml};

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
    let dir : String = env::var("CONFIG_DIR").unwrap();
    let path = format!("{}/emojibot.toml", dir);

    let config = read_toml::<Config>(&path).unwrap();
    debug!("{:?}", config);

    config
}

pub fn debug_write_config(config : Config) {
    let dir : String = env::var("STORE_DIR").unwrap();
    let path = format!("{}/store.toml", dir);

    write_toml(&path, config).unwrap();
}