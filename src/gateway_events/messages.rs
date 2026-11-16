//! Reference: https://docs.discord.com/developers/events/gateway-events#messages

use serde::{Deserialize, Serialize};
use crate::structs::base::Snowflake;

#[derive(Serialize, Deserialize)]
struct MessageCreate {
    guild_id: Snowflake,
    
}