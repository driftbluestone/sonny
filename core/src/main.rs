extern crate serenity;
extern crate ctor;
extern crate registry;
extern crate reg;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use reg::event;

mod discord;

#[tokio::main]
async fn main() {
    let client =
        Client::builder(TOKEN, GatewayIntents::all()).event_handler(discord::Handler).await;

    client.unwrap().start().await.unwrap();
}

static TOKEN: &str = "";

#[event(message)]
async fn on_message(context: Context, msg: Message) {
    if msg.content == "!ping" {
        let _ = msg.channel_id.say(&context, "Pong!").await;
    }
}
