pub mod models;
pub mod socket;
pub mod ws;

use std::collections::HashSet;
use std::{env, vec};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::offset::Utc;
use serenity::all::UserId;
use serenity::http::Http;
use serenity::{all::GuildId, async_trait};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::gateway::ActivityData;
use serenity::prelude::*;
use serenity::framework::standard::macros::{group, help};
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandResult,
    Configuration,
    HelpOptions,
    StandardFramework,
};
use tracing::error;
use ws::kill_feed;

#[group("collector")]
struct Collector;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

struct Bot {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built succesfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    kill_feed(&ctx1).await;
                }
            });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_activity_to_current_time(&ctx2);
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

fn set_activity_to_current_time(ctx: &Context) {
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    ctx.set_activity(Some(ActivityData::playing(formatted_time)));
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let bot_id = match http.get_current_user().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access user info: {:?}", why),
    };

    let framework = StandardFramework::new().help(&MY_HELP).group(&COLLECTOR_GROUP);

    framework.configure(
        Configuration::new()
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~")
            .delimiters(vec![", ", ","]),
    );

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Bot {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
