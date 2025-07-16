use serde::{Deserialize, Serialize};

const URL_BASE: &str = "https://esi.evetech.net/latest/";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub security_status: f64,
}

impl Character {
    pub async fn get_character(id: i64) -> Result<Character, reqwest::Error> {
        let get_url = format!("{}characters/{}/?datasource=tranquility", URL_BASE, id);

        let response: Character = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
}

impl Ship {
    pub async fn get_ship(id: i64) -> Result<Ship, reqwest::Error> {
        let get_url = format!(
            "{}universe/types/{}/?datasource=tranquility&language=en",
            URL_BASE, id
        );

        let response: Ship = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub constellation_id: i64,
    pub name: String,
    pub security_status: f64,
}

impl System {
    pub async fn get_system(id: i64) -> Result<System, reqwest::Error> {
        let get_url = format!(
            "{}universe/systems/{}/?datasource=tranquility&language=en",
            URL_BASE, id
        );

        let response: System = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constellation {
    pub name: String,
    pub region_id: i64,
}

impl Constellation {
    pub async fn get_constellation(id: i64) -> Result<Constellation, reqwest::Error> {
        let get_url = format!(
            "{}universe/constellations/{}/?datasource=tranquility&language=en",
            URL_BASE, id
        );

        let response: Constellation = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
}

impl Region {
    pub async fn get_region(id: i64) -> Result<Region, reqwest::Error> {
        let get_url = format!(
            "{}universe/regions/{}/?datasource=tranquility&language=en",
            URL_BASE, id
        );

        let response: Region = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Corporation {
    pub name: String,
}

impl Corporation {
    pub async fn get_corp(id: i64) -> Result<Corporation, reqwest::Error> {
        let get_url = format!("{}corporations/{}/?datasource=tranquility", URL_BASE, id);

        let response: Corporation = reqwest::get(get_url).await?.json().await?;

        Ok(response)
    }
}
