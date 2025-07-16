use log::{error, info};
use std::collections::HashMap;
use tokio::fs;
use toml;

use crate::ws::{ServerConfig, SERVER_CONFIGS};

const CONFIG_FILE: &str = "server_configs.toml";

pub async fn save_configs() {
    let configs = SERVER_CONFIGS.read().await;
    info!("Attempting to save {} configs", configs.len());

    let string_keyed_configs: HashMap<String, ServerConfig> = configs
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();

    match toml::to_string_pretty(&string_keyed_configs) {
        Ok(toml_string) => {
            info!("Generated TOML: {}", toml_string);
            match fs::write(CONFIG_FILE, toml_string).await {
                Ok(_) => info!("Successfully wrote to {}", CONFIG_FILE),
                Err(e) => error!("Failed to write file: {}", e),
            }
        }
        Err(e) => {
            error!("Failed to serialize to TOML: {}", e);
        }
    }
}

pub async fn load_configs() {
    if let Ok(data) = fs::read_to_string(CONFIG_FILE).await {
        if let Ok(string_configs) = toml::from_str::<HashMap<String, ServerConfig>>(&data) {
            let configs: HashMap<u64, ServerConfig> = string_configs
                .into_iter()
                .filter_map(|(k, v)| k.parse::<u64>().ok().map(|guild_id| (guild_id, v)))
                .collect();

            let mut server_configs = SERVER_CONFIGS.write().await;
            *server_configs = configs.clone();
            info!("loaded {} server configurations", configs.len());
        }
    }
}
