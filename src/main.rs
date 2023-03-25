use std::collections::HashMap;
use std::{env, fs, io};

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct SlacordConfig {
    responses: HashMap<String, Vec<String>>,
}

#[group]
#[commands(ping)]
struct General;

struct Handler {
    config: SlacordConfig,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.webhook_id.is_some() {
            return;
        }

        let tokens: Vec<&str> = msg.content.split(" ").collect();

        for (key, value) in &self.config.responses {
            for &token in &tokens {
                if !token.eq(key.as_str()) {
                    continue;
                }

                println!("'{}' matched {}: {:?}", token, key, value);

                let index = rand::random::<usize>() % value.len();

                if let Err(why) = msg.channel_id.say(&ctx.http, &value[index]).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
}

fn read_config() -> Result<SlacordConfig, io::Error> {
    println!("Loading the config from config.json");

    let config_raw = match fs::read_to_string("config.json") {
        Ok(txt) => txt,
        Err(e) => return Err(e),
    };

    let config: SlacordConfig = serde_json::from_str(config_raw.as_str())?;
    println!("Successfully loaded: {:?}", config);

    return Ok(config);
}

#[tokio::main]
async fn main() {
    let config = match read_config() {
        Ok(c) => c,
        Err(e) => panic!("Failed to read config.json: {:?}", e),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler { config: config })
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Slacordbot is up and running");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
