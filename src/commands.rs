use log::{error, info, warn};
use serenity::all::{
    CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::client::Context;

use crate::configs::save_configs;
use crate::ws::{SERVER_CONFIGS, ServerConfig};

pub async fn setup_command(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id.get(),
        None => {
            send_error_response(ctx, command, "This command can only be used in a server").await;
            return;
        }
    };

    let follow_id_str = match command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "follow_id")
        .and_then(|opt| opt.value.as_str())
    {
        Some(val) => val.trim(),
        None => {
            send_error_response(ctx, command, "Please provided a follow_id").await;
            return;
        }
    };

    let follow_id = match validate_follow_id(follow_id_str) {
        Ok(id) => id,
        Err(error_msg) => {
            send_error_response(ctx, command, &error_msg).await;
            return;
        }
    };

    let channel_id = command.channel_id.get();

    info!(
        "setting up: guild_id={}, follow_id={}, channel_id={}",
        guild_id, follow_id, channel_id
    );

    {
        let mut configs = SERVER_CONFIGS.write().await;

        let config = configs.entry(guild_id).or_insert(ServerConfig {
            follow_ids: Vec::new(),
            channel_id,
        });

        if !config.follow_ids.contains(&follow_id) {
            config.follow_ids.push(follow_id)
        }

        config.channel_id = channel_id;
    }
    save_configs().await;

    let response =
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
            format!("✅ Draugur configured! Now tracking ID: {}", follow_id),
        ));

    if let Err(why) = command.create_response(&ctx.http, response).await {
        error!("Cannot repond to slash command: {}", why);
    }
}

fn validate_follow_id(input: &str) -> Result<i64, String> {
    if input.is_empty() {
        return Err("Follow ID cannot be empty".to_string());
    }

    if input.len() > 14 {
        return Err("Follow ID is too long".to_string());
    }

    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err("Follow ID must contain only numbers".to_string());
    }

    match input.parse::<i64>() {
        Ok(id) => {
            if id <= 0 {
                Err("Follow ID must be a positive number".to_string())
            } else if id > 9999999999 {
                Err("Follow ID is too large".to_string())
            } else {
                Ok(id)
            }
        }
        Err(_) => Err("Invalid follow ID format".to_string()),
    }
}

pub async fn remove_command(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id.get(),
        None => {
            send_error_response(ctx, command, "This command can only be used in a server").await;
            return;
        }
    };

    let follow_id_str = match command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "follow_id")
        .and_then(|opt| opt.value.as_str())
    {
        Some(val) => val.trim(),
        None => {
            send_error_response(ctx, command, "Please provide a follow_id to remove").await;
            return;
        }
    };

    let follow_id = match validate_follow_id(follow_id_str) {
        Ok(id) => id,
        Err(error_msg) => {
            send_error_response(ctx, command, &error_msg).await;
            return;
        }
    };

    let removed = {
        let mut configs = SERVER_CONFIGS.write().await;
        if let Some(config) = configs.get_mut(&guild_id) {
            if let Some(pos) = config.follow_ids.iter().position(|&x| x == follow_id) {
                config.follow_ids.remove(pos);

                if config.follow_ids.is_empty() {
                    configs.remove(&guild_id);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    };

    if removed {
        save_configs().await;
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(format!("Removed ID {} from tracking list", follow_id))
                .ephemeral(true),
        );

        if let Err(why) = command.create_response(&ctx.http, response).await {
            warn!("Cannot respond to slash command: {}", why);
        }
    } else {
        send_error_response(
            ctx,
            command,
            &format!("ID {} is not in your tracking list.", follow_id),
        )
        .await;
    }
}

pub async fn list_command(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id.get(),
        None => {
            send_error_response(ctx, command, "Command can only be used in servers").await;
            return;
        }
    };

    let configs = SERVER_CONFIGS.read().await;
    let response_content = if let Some(config) = configs.get(&guild_id) {
        if config.follow_ids.is_empty() {
            "No IDs are currently being tracked.".to_string()
        } else {
            let ids_list = config
                .follow_ids
                .iter()
                .map(|id| format!("• `{}`", id))
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "**Currently tracking {} ID(s):**\n{}\n\n*Posting to <#{}>*",
                config.follow_ids.len(),
                ids_list,
                config.channel_id
            )
        }
    } else {
        "No IDs are currently being tracked.".to_string()
    };

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(response_content)
            .ephemeral(true),
    );

    if let Err(why) = command.create_response(&ctx.http, response).await {
        println!("Cannot respond to slash command: {}", why);
    }
}

async fn send_error_response(ctx: &Context, command: &CommandInteraction, message: &str) {
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("❌ {}", message))
            .ephemeral(true),
    );

    if let Err(why) = command.create_response(&ctx.http, response).await {
        warn!("Cannot respond to slash command: {}", why);
    }
}
