mod structs {
    pub mod base {
        pub mod snowflake;
        mod base;
        mod emotes;
        mod roles;
        
    }
    mod user;
}
mod ws {
    pub mod intents;
    pub mod bot;
    mod structs;
    mod opcodes;
    mod http;
}
mod gateway_events {
    mod application_commands;
    mod messages;
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

/**
 * We'll also want rate limiting prevention, Discord limits bots to 50 REST requests/sec,
 * and Cloudflare limits invalid requests to 10000 requests per 10 minutes. (40x's).
 * Assuming we program this well, that shouldn't be an issue. Also, apparently, interaction
 * responses/followups are excluded from the 50 req/sec limit.
 * 
 * We could start buffering events into an async queue if a counter exceeds a safety
 * value. (i.e. every request adds 1 to a counter, but we subtract 1 every 25 ms, if it
 * goes above ~40, start buffering)
 * 
 * https://docs.discord.com/developers/topics/rate-limits
 * "Because rate limits depend on a variety of factors and are subject to change, rate
 * limits should not be hard coded into your app. Instead, your app should parse response
 * headers to prevent hitting the limit, and to respond accordingly in case you do."
 * ...oh.
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token: String = std::fs::read_to_string("./TOKEN.txt").unwrap();
    let bot: Bot = Bot::new(&token, intents::ALL);
    let _ = bot.start_blocking().await;
    Ok(())
}