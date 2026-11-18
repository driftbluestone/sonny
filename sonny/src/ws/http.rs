use reqwest::{RequestBuilder, header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT}};

use crate::ws::bot::Bot;

const API_URL: &str = "https://discord.com/api/v10";
const API_URL_LEN: usize = API_URL.len();

macro_rules! api {
    ($e:tt) => {{
        let mut result = String::with_capacity(API_URL_LEN + $e.len());
        result.push_str(API_URL);
        result.push_str(&$e);
        result
    }};
}

trait ApplyHeaders {
    fn defaults(self, bot: &Bot) -> Self;
}
impl ApplyHeaders for RequestBuilder {
    fn defaults(self, bot: &Bot) -> Self {
        self.header(AUTHORIZATION, &bot.auth)
        .header(USER_AGENT, "DiscordBot (https://github.com/driftbluestone/sonny, 1.0.0)") // make url and version optionally customizable later
        .header(CONTENT_TYPE, "application/json")
    }
}

#[allow(dead_code)]
pub trait Http {
    async fn send_message(&self, channel_id: u64, text: Option<String>, ) -> Result<reqwest::Response, reqwest::Error>; // eventually will need support for components
}

impl Http for Bot {
    async fn send_message(&self, channel_id: u64, text: Option<String>, ) -> Result<reqwest::Response, reqwest::Error> {
        let payload = serde_json::json!({
        "content": text,
        });
        let url = format!("/channels/{}/messages", channel_id);
        self.http
        .post(api!(url))
        .defaults(self)
        .body(payload.to_string())
        .send().await
    }
}