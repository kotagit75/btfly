use std::{env, sync::LazyLock};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct InternalConfig {
    pub p2p_port: u16,
    pub vdf_difficulty: u64,
}

pub struct Config {
    pub api_port: u16,
    pub cors_allow_port: u16,
    pub internal_config: InternalConfig,
}

const INTERNAL_CONFIG_JSON: &str = include_str!("config.json");

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let api_port = if let Ok(Ok(val)) = env::var("API_PORT").map(|port| port.parse::<u16>()) {
        val
    } else {
        8080
    };
    let cors_allow_port =
        if let Ok(Ok(val)) = env::var("CORS_ALLOW_PORT").map(|port| port.parse::<u16>()) {
            val
        } else {
            3000
        };

    let internal_config: InternalConfig = match serde_json::from_str(INTERNAL_CONFIG_JSON) {
        Ok(config) => config,
        Err(_) => InternalConfig {
            p2p_port: 62697,
            vdf_difficulty: 100000,
        },
    };
    Config {
        api_port,
        cors_allow_port,
        internal_config,
    }
});
