extern crate reg;
extern crate serenity;
extern crate ctor;
extern crate registry;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
mod discord;
use reg::event;

#[tokio::main]
async fn main() {
    let client =
        Client::builder(TOKEN, GatewayIntents::all()).event_handler(discord::Handler).await;

    client.unwrap().start().await.unwrap();
}

static TOKEN: &str = "";

#[event(message)]
async fn on_message(context: &mut Context, msg: &mut Message) {
    if msg.content == "!ping" {
        let _ = msg.channel_id.say(&*context, "Pong!").await;
    }
}
