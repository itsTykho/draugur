use log::{error, info};
use serenity::all::{
    CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::client::Context;

use crate::configs::save_configs;
use crate::ws::{ServerConfig, SERVER_CONFIGS};

pub async fn setup_command(ctx: &Context, command: &CommandInteraction) {
    let guild_id = command.guild_id.unwrap().get();

    let follow_id = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "follow_id")
        .and_then(|opt| opt.value.as_str())
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let channel_id = command.channel_id.get();

    info!(
        "setting up: guild_id={}, follow_id={}, channel_id={}",
        guild_id, follow_id, channel_id
    );

    {
        let mut configs = SERVER_CONFIGS.write().await;
        configs.insert(
            guild_id,
            ServerConfig {
                follow_id,
                channel_id,
            },
        );
        info!("config inserted, total configs: {}", configs.len());
    }
    save_configs().await;

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Draugur configured!"),
    );

    if let Err(why) = command.create_response(&ctx.http, response).await {
        error!("Cannot repond to slash command: {}", why);
    }
}
