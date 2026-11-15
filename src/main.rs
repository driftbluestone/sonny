mod structs {
    mod base;
    mod user;
}
mod ws {
    pub mod intents;
    pub mod bot;
    mod structs;
    mod opcodes;
    mod http;
}

use std::error::Error;

use crate::ws::{bot::Bot, intents};
/**
 * Thoughts on design
 * Since the http client is bound to the Bot struct, we should probably embed
 * a reference to the bot somewhere in every "interaction" type struct.
 * like discord py's ctx.reply(), if ctx contains a reference to the bot struct then
 * reply just formats a function call to bot.send_message or something.
 * should be a private function though. or in some trait somewhere.
 * 
 * also the USER_AGENT stuff should be configurable. optionally, though. 
 * 
 * caching. how does caching work? isn't that a state management nightmare?
 * do you just update the information on every user every time an event is recieved?
 * that seems... inefficient.
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token: String = std::fs::read_to_string("./TOKEN.txt").unwrap();
    let bot: Bot = Bot::new(&token, intents::ALL);
    let _ = bot.start_blocking().await;
    Ok(())
}