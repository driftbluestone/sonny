mod ws {
    pub mod intents;
    pub mod bot;
    pub mod structs;
    mod opcodes;
}

use std::error::Error;

use crate::ws::{bot::Bot, intents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = std::fs::read_to_string("./TOKEN.txt").unwrap();
    let bot: Bot = Bot::new(&token, intents::ALL);
    let _ = bot.start_blocking().await;
    Ok(())
}