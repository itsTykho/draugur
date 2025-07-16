pub mod commands;
pub mod configs;
pub mod esi;
pub mod helpers;
pub mod models;
pub mod msg;
pub mod ws;

use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use log::{error, info};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, Interaction};
use serenity::gateway::{ActivityData, ShardManager};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{all::GuildId, async_trait};

use commands::setup_command;
use configs::load_configs;
use helpers::get_most_expensive_recent_kill;
use ws::kill_feed;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Bot {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "setup" => setup_command(&ctx, &command).await,
                _ => {}
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let command = CreateCommand::new("setup")
            .description("Setup killmail tracking for this server")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "follow_id",
                    "corporation or alliance id to track",
                )
                .required(true),
            );

        if let Err(why) = ctx.http.create_global_command(&command).await {
            error!("cannot create slash command: {}", why);
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache built succesfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                kill_feed(&ctx1).await;
            });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_activity_to_expensive_kill(&ctx2).await;
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn set_activity_to_expensive_kill(ctx: &Context) {
    if let Some((victim_name, ship_name, total_value)) = get_most_expensive_recent_kill().await {
        let value_in_billions = total_value / 1_000_000_000.0;
        let activity_text = if value_in_billions >= 1.0 {
            format!(
                "💀 {}'s {} ({}B ISK)",
                victim_name,
                ship_name,
                value_in_billions.round() as u64
            )
        } else {
            format!(
                "💀 {}'s {} ({}M ISK)",
                victim_name,
                ship_name,
                (total_value / 1_000_000.0).round() as u64
            )
        };
        ctx.set_activity(Some(ActivityData::watching(activity_text)));
    } else {
        ctx.set_activity(Some(ActivityData::listening("Zkill...")));
    }
}

#[tokio::main]
async fn main() {
    colog::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    load_configs().await;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone())
    }

    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
