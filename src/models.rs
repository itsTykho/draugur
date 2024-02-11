use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Killmail {
    pub attackers: Vec<Attacker>,
    #[serde(rename = "killmail_id")]
    pub killmail_id: i64,
    #[serde(rename = "killmail_time")]
    pub killmail_time: String,
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i64,
    pub victim: Victim,
    pub zkb: Zkb,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attacker {
    #[serde(default = "default_to_zero")]
    pub killmail_id: i64,
    #[serde(rename = "alliance_id")]
    #[serde(default = "default_to_zero")]
    pub alliance_id: i64,
    #[serde(rename = "character_id")]
    #[serde(default = "default_to_zero")]
    pub character_id: i64,
    #[serde(rename = "corporation_id")]
    #[serde(default = "default_to_zero")]
    pub corporation_id: i64,
    #[serde(rename = "damage_done")]
    #[serde(default = "default_to_float")]
    pub damage_done: f64,
    #[serde(rename = "final_blow")]
    pub final_blow: bool,
    #[serde(rename = "security_status")]
    pub security_status: f64,
    #[serde(rename = "ship_type_id")]
    #[serde(default = "default_to_zero")]
    pub ship_type_id: i64,
    #[serde(rename = "weapon_type_id")]
    #[serde(default = "default_to_zero")]
    pub weapon_type_id: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Victim {
    #[serde(default = "default_to_zero")]
    pub killmail_id: i64,
    #[serde(rename = "character_id")]
    #[serde(default = "default_to_zero")]
    pub character_id: i64,
    #[serde(rename = "corporation_id")]
    #[serde(default = "default_to_zero")]
    pub corporation_id: i64,
    #[serde(rename = "damage_taken")]
    pub damage_taken: f64,
    #[serde(rename = "faction_id")]
    #[serde(default = "default_to_zero")]
    pub faction_id: i64,
    #[serde(rename = "ship_type_id")]
    #[serde(default = "default_to_zero")]
    pub ship_type_id: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zkb {
    #[serde(default = "default_to_zero")]
    pub killmail_id: i64,
    #[serde(rename = "locationID")]
    pub location_id: i64,
    pub hash: String,
    pub fitted_value: f64,
    pub dropped_value: f64,
    pub destroyed_value: f64,
    pub total_value: f64,
    pub points: f64,
    pub npc: bool,
    pub solo: bool,
    pub awox: bool,
}

fn default_to_zero() -> i64 {
    0
}

fn default_to_float() -> f64 {
    0.0
}
