extern crate serenity;
extern crate reg;

use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use reg::{event, cmd, acmd, Handler};

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
async fn ppp(ctx: serenity::client::Context, msg: Message) {
    let _ = msg.reply_mention(&ctx, "{msg}").await;
}

#[acmd(test)]
async fn g(_ctx: Context, _interaction: Interaction) {

}

#[event(interaction_create)]
async fn test(ctx: Context, interaction: Interaction) {
    println!("e");
    if let Interaction::Command(command) = interaction {
        let data = CreateInteractionResponseMessage::new().content("e");
        let builder = CreateInteractionResponse::Message(data);
        let _ = command.create_response(&ctx, builder).await;
    }
}
