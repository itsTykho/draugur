use serenity::all::{ChannelId, CreateEmbedAuthor};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::client::Context;
use serenity::model::Colour;

use crate::esi;
use crate::helpers::{format_isk, format_time, get_final_blower};
use crate::models::{Killmail, Zkb};

const GREEN_KILL: Colour = Colour::from_rgb(50, 230, 175);
const RED_LOSS: Colour = Colour::from_rgb(180, 50, 110);

pub async fn create_msg(ctx: &Context, channel_id: u64, kill_type: String, km: Killmail, zkb: Zkb) {
    let color = if kill_type == "loss" {
        RED_LOSS
    } else {
        GREEN_KILL
    };

    let author = if kill_type == "loss" { "Loss" } else { "Kill" };

    let badge_url: String = if km.victim.alliance_id.unwrap_or(0) == 0 {
        format!(
            "https://images.evetech.net/corporations/{}/logo?size=64",
            km.victim.corporation_id
        )
    } else {
        format!(
            "https://images.evetech.net/alliances/{}/logo?size=64",
            km.victim.alliance_id.unwrap_or(0)
        )
    }
    .to_string();

    let ship_badge = format!(
        "https://images.evetech.net/types/{}/render?size=64",
        km.victim.ship_type_id
    );
    let url = format!("https://zkillboard.com/kill/{}/", km.killmail_id);

    let fmt_time = format_time(km.killmail_time);

    let footer_str = format!("{} • {}", format_isk(zkb.total_value), fmt_time);
    let footer = CreateEmbedFooter::new(footer_str);

    let author = CreateEmbedAuthor::new(author).icon_url(badge_url).url(&url);

    let vic = esi::Character::get_character(km.victim.character_id).await;
    let vic_result_name = match vic {
        Ok(vic_name) => vic_name.name,
        _ => "Unknown".into(),
    };

    let vic_ship = esi::Ship::get_ship(km.victim.ship_type_id).await;
    let vic_ship_name = match vic_ship {
        Ok(shp) => shp.name,
        _ => "Unknown".into(),
    };

    let system = esi::System::get_system(km.solar_system_id).await;
    let system_name = match system {
        Ok(ref sys) => &sys.name,
        _ => "Unknown",
    };

    let const_id = match system {
        Ok(ref const_id) => const_id.constellation_id,
        _ => 0,
    };

    let constellation = esi::Constellation::get_constellation(const_id.to_owned()).await;
    let region = match constellation {
        Ok(constel) => constel.region_id,
        _ => 0,
    };

    let region = esi::Region::get_region(region).await;
    let region_name = match region {
        Ok(reg) => reg.name,
        _ => "Unknown".into(),
    };

    let solo = if zkb.solo == true || km.attackers.len() == 1 {
        "solo!".to_string()
    } else if km.attackers.len() == 2 {
        format!("with 1 friend")
    } else {
        format!("with {} friends", km.attackers.len() - 1)
    };

    let (fb, fb_id, fb_ship_name, fb_corp_id) = get_final_blower(&km.attackers).await;
    let fb_corp = esi::Corporation::get_corp(fb_corp_id).await;
    let fb_corp_name = match fb_corp {
        Ok(fcn) => fcn.name,
        _ => "Unknown".into(),
    };

    let vic_corp = esi::Corporation::get_corp(km.victim.corporation_id).await;
    let vic_corp_name = match vic_corp {
        Ok(vcn) => vcn.name,
        _ => "Unknown".into(),
    };

    let embed = CreateEmbed::new()
        .colour(color)
        .title(format!(
            "{}'s {} was destroyed in {} ({})",
            vic_result_name, vic_ship_name, system_name, region_name
        ))
        .url(&url)
        .author(author)
        .description(format!(
            "**{} ({})** lost their {} to **[{}](https://zkillboard.com/character/{}/) ({})** flying a {} {}",
            vic_result_name,
            vic_corp_name,
            vic_ship_name,
            fb,
            fb_id,
            fb_corp_name,
            fb_ship_name,
            solo
        ))
        .thumbnail(ship_badge)
        .footer(footer);

    let builder = CreateMessage::new().embed(embed);

    let msg = ChannelId::new(channel_id)
        .send_message(&ctx.http, builder)
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}")
    }
}
