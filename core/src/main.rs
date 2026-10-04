extern crate serenity;
extern crate reg;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use reg::*;

#[tokio::main]
async fn main() {
    let mut client =
        Client::builder(TOKEN, GatewayIntents::all()).event_handler(reg::Handler).await.unwrap();

    client.start().await.unwrap();
}

static TOKEN: &str = "";

#[acmd(test)]
async fn g(ctx: Context, interaction: CommandInteraction, num: Option<User>) {
    if let Some(user) = num {
        let _ = interaction.response(&ctx, &format!("hi {}", user.name), false).await;
    } else {
        let _ = interaction.response(&ctx, &format!("hi"), false).await;
    }
    
}