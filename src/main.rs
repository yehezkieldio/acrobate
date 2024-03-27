use dotenv::dotenv;
use serenity::all::EditMessage;
use std::env;

use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{all::Ready, async_trait};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

async fn fetch_todo() -> Result<Todo, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let response = reqwest::get(url).await?.text().await?;
    let todo: Todo = serde_json::from_str(&response).unwrap();
    Ok(todo)
}

static PREFIX: &str = "acro!";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            let command = msg.content.trim_start_matches(PREFIX);
            match command {
                "ping" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                "test" => {
                    let mut message = msg
                        .channel_id
                        .say(&ctx.http, "Testing request...")
                        .await
                        .unwrap();

                    let todo = fetch_todo().await.unwrap();
                    println!("{:?}", todo);
                    let todo_message = format!("Todo: {}", todo.title);

                    let builder = EditMessage::new().content(todo_message);
                    message.edit(&ctx.http, builder).await.unwrap()
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating Client.");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
