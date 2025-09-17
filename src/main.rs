use std::fs;
use serde::Deserialize;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Deserialize)]
struct BotConfig {
    token: String,
    prefix: String,
}

#[derive(Deserialize)]
struct Config {
    bot: BotConfig,
}

fn load_config() -> Config {
    let content = std::fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::from_str(&content)
        .expect("Invalid format in config.toml")
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == format!("{}ping", load_config().bot.prefix) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = load_config().bot.token;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}