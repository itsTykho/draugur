use crate::helpers::{get_vic_info, track_recent_kill};
use crate::models::Killmail;
use crate::models::Zkb;
use crate::msg::create_msg;

use log::{debug, error};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::RwLock;

pub static SERVER_CONFIGS: Lazy<RwLock<HashMap<u64, ServerConfig>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub follow_ids: Vec<i64>,
    pub channel_id: u64,
}

pub async fn kill_feed(ctx: &Context) {
    let url = format!("https://zkillredisq.stream/listen.php?queueID=draugur");

    let client = reqwest::Client::new();
    loop {
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(text) = response.text().await {
                    if let Ok(redis_response) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(package) = redis_response.get("package") {
                            if package.is_null() {
                                continue;
                            }

                            if let Some(killmail_data) = package.get("killmail") {
                                if let Some(zkb_data) = package.get("zkb") {
                                    debug!("ZKB DATA: {:?}", zkb_data);
                                    if let Ok(parsed) =
                                        serde_json::from_value::<Killmail>(killmail_data.clone())
                                    {
                                        if let Ok(zkb) =
                                            serde_json::from_value::<Zkb>(zkb_data.clone())
                                        {
                                            let (vic, vic_ship) =
                                                get_vic_info(parsed.clone()).await;

                                            track_recent_kill(
                                                parsed.killmail_id,
                                                zkb.total_value,
                                                vic,
                                                vic_ship,
                                            )
                                            .await;
                                            let mut a_ids: Vec<i64> = Vec::new();
                                            for attacker in parsed.attackers.iter() {
                                                if let Some(alliance_id) = attacker.alliance_id {
                                                    a_ids.push(alliance_id);
                                                }
                                            }
                                            let mut corp_ids: Vec<i64> = Vec::new();
                                            for attacker in parsed.attackers.iter() {
                                                corp_ids.push(attacker.corporation_id);
                                            }
                                            let configs = SERVER_CONFIGS.read().await;
                                            for (_guild_id, config) in configs.iter() {
                                                if let Some(kill_type) =
                                                    should_track(&parsed, &config.follow_ids[..])
                                                {
                                                    create_msg(
                                                        ctx,
                                                        config.channel_id,
                                                        kill_type,
                                                        parsed.clone(),
                                                        zkb.clone(),
                                                    )
                                                    .await;
                                                }
                                            }
                                            drop(configs);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("request failed: {}", e);
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}

fn should_track(km: &Killmail, follow_ids: &[i64]) -> Option<String> {
    if follow_ids.contains(&km.victim.character_id)
        || follow_ids.contains(&km.victim.corporation_id)
        || km
            .victim
            .alliance_id
            .map_or(false, |id| follow_ids.contains(&id))
    {
        return Some("loss".to_string());
    }

    for attacker in &km.attackers {
        if follow_ids.contains(&attacker.character_id)
            || follow_ids.contains(&attacker.corporation_id)
            || attacker
                .alliance_id
                .map_or(false, |id| follow_ids.contains(&id))
        {
            return Some("kill".to_string());
        }
    }

    if follow_ids.contains(&km.solar_system_id) {
        return Some("kill".to_string());
    }

    if follow_ids.contains(&km.victim.ship_type_id) {
        return Some("loss".to_string());
    }

    for attacker in &km.attackers {
        if follow_ids.contains(&attacker.ship_type_id) {
            return Some("kill".to_string());
        }
    }

    None
}
