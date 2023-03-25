use std::collections::HashMap;
use std::{env, fs, io};

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::{Emoji, EmojiId, GuildId};
use serenity::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    triggers: Vec<String>,
    responses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SlacordConfig {
    responses: Vec<Response>,
}

#[group]
#[commands(ping)]
struct General;

struct Handler {
    config: SlacordConfig,
    emojis: Mutex<HashMap<u64, Vec<Emoji>>>,
}

impl Handler {
    async fn fetch_emojis(&self, ctx: &Context, id: &u64, force: bool) {
        let mut emoji_map = self.emojis.lock().await;
        if let Some(_) = emoji_map.get(id) {
            if !force {
                return;
            }
        }

        if let Ok(emojis) = ctx.http.get_emojis(*id).await {
            println!("Updated the emoji list of {}", *id);
            emoji_map.insert(*id, emojis);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.webhook_id.is_some() {
            return;
        }

        let mut guild_id: u64 = 0;
        match msg.guild_id {
            None => println!("No guild ID!"),
            Some(id) => guild_id = id.as_u64().clone(),
        }

        if guild_id != 0 {
            self.fetch_emojis(&ctx, &guild_id, false).await;
        }

        let tokens: Vec<&str> = msg.content.split(" ").collect();

        for response in &self.config.responses {
            for trigger in &response.triggers {
                for &token in &tokens {
                    if !token.eq(trigger.as_str()) {
                        continue;
                    }

                    let index = rand::random::<usize>() % response.responses.len();
                    let mut response = response.responses[index].clone();

                    println!("'{}' matched {}: {:?}", token, trigger, response);

                    if let Some(emojis) = self.emojis.lock().await.get(&guild_id) {
                        for emoji in emojis {
                            let human_repr = format!(":{}:", emoji.name);
                            response =
                                response.replace(human_repr.as_str(), emoji.to_string().as_str());
                        }
                    } else {
                        println!("Emoji list is not available. Sending the response without replacing emoji.")
                    }

                    println!("Replaced: {}", trigger);

                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }

    async fn guild_emojis_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        _current_state: HashMap<EmojiId, Emoji>,
    ) {
        // TODO: updae the internal emoji list with _current_state
        println!("Emojis got updated, updating the emoji list");
        self.fetch_emojis(&ctx, guild_id.as_u64(), true).await;
    }
}

fn read_config() -> Result<SlacordConfig, io::Error> {
    println!("Loading the config from config.json");

    let config_raw = match fs::read_to_string("config.json") {
        Ok(txt) => txt,
        Err(e) => return Err(e),
    };

    let config: SlacordConfig = serde_json::from_str(config_raw.as_str())?;
    println!("Successfully loaded: {} reactions", config.responses.len());

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
        .event_handler(Handler {
            config: config,
            emojis: Mutex::new(HashMap::new()),
        })
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
