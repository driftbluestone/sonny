extern crate serenity;
extern crate reg;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use reg::{cmd, Handler};

#[tokio::main]
async fn main() {
    let mut client =
        Client::builder(TOKEN, GatewayIntents::all()).event_handler(Handler).await.unwrap();

    client.start().await.unwrap();
}

static TOKEN: &str = "";

#[cmd(ping)]
async fn ping(ctx: Context, msg: Message) {
    let _ = msg.reply_mention(&ctx, "Pong!").await;
}

#[cmd(ping me)]
async fn ppp(ctx: Context, msg: Message) {
    let _ = msg.reply_mention(&ctx, "get pong'd").await;
}
