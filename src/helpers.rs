use chrono::{DateTime, NaiveDateTime, Utc};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use tokio::sync::Mutex;

use crate::models::Killmail;
use crate::{esi, models::Attacker};

pub async fn get_final_blower(attackers: &Vec<Attacker>) -> (String, i64, String, i64) {
    let mut fb_corp_id = 0;
    let mut final_blower_id = 0;
    let mut fb_ship_id = 0;
    for a in attackers {
        (final_blower_id, fb_ship_id, fb_corp_id) = if a.final_blow == true {
            (a.character_id, a.ship_type_id, a.corporation_id)
        } else {
            continue;
        };
    }

    let fb = esi::Character::get_character(final_blower_id).await;
    let final_blower = match fb {
        Ok(fber) => fber.name,
        _ => "Unknown".into(),
    };
    let fb_ship = esi::Ship::get_ship(fb_ship_id).await;
    let fb_ship_name = match fb_ship {
        Ok(fbs) => fbs.name,
        _ => "Unknown".into(),
    };

    (final_blower, final_blower_id, fb_ship_name, fb_corp_id)
}

pub fn format_isk(isk: f64) -> String {
    let _value = "0 ISK";

    let value = if isk >= 1000000000f64 {
        format!("{:.0}B ISK", (isk / 100000000f64) / 10f64)
    } else if isk >= 1000000f64 {
        format!("{:.0}M ISK", (isk / 100000f64) / 10f64)
    } else {
        format!("{:.0}K ISK", (isk / 100f64) / 10f64)
    }
    .to_string();

    value
}

pub fn format_time(km_time: String) -> String {
    let fmt_time = NaiveDateTime::parse_from_str(&km_time, "%Y-%m-%dT%H:%M:%SZ");
    let ft = match fmt_time {
        Ok(ftutc) => ftutc.to_string(),
        _ => km_time,
    };

    ft
}

#[derive(Clone)]
pub struct RecentKill {
    pub killmail_id: i64,
    pub total_value: f64,
    pub timestamp: DateTime<Utc>,
    pub victim_name: String,
    pub victim_ship_name: String,
}

pub static RECENT_KILLS: Lazy<Mutex<VecDeque<RecentKill>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));

pub async fn track_recent_kill(
    killmail_id: i64,
    total_value: f64,
    victim_name: String,
    victim_ship_name: String,
) {
    let mut kills = RECENT_KILLS.lock().await;
    let now = Utc::now();

    kills.push_back(RecentKill {
        killmail_id,
        total_value,
        timestamp: now,
        victim_name,
        victim_ship_name,
    });

    let ten_minutes_ago = now - chrono::Duration::minutes(10);
    while let Some(front) = kills.front() {
        if front.timestamp < ten_minutes_ago {
            kills.pop_front();
        } else {
            break;
        }
    }
}

pub async fn get_most_expensive_recent_kill() -> Option<(String, String, f64)> {
    let kills = RECENT_KILLS.lock().await;
    kills
        .iter()
        .max_by(|a, b| {
            a.total_value
                .partial_cmp(&b.total_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|kill| {
            (
                kill.victim_name.clone(),
                kill.victim_ship_name.clone(),
                kill.total_value,
            )
        })
}

pub async fn get_vic_info(km: Killmail) -> (String, String) {
    let vic = esi::Character::get_character(km.victim.character_id).await;
    let victim_name = match vic {
        Ok(character) => character.name,
        _ => "Unknown".to_string(),
    };

    let vic_ship = esi::Ship::get_ship(km.victim.ship_type_id).await;
    let ship_name = match vic_ship {
        Ok(ship) => ship.name,
        _ => "Unknown".to_string(),
    };

    (victim_name, ship_name)
}
