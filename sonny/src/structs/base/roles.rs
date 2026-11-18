use serde::Deserializer;

use crate::structs::base::snowflake::*;

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object
#[derive(Serialize, Deserialize)]
pub struct RoleBase {
    id: Snowflake,
    name: String,
    colors: Color,
    hoist: bool,
    icon: NullableOption<String>,
    unicode_emoji: NullableOption<String>,
    position: u64,
    permissions: String, // apparently it's a "permission bit set", look into that later
    managed: bool,
    mentionable: bool,
    tags: Option<RoleTags>,
    flags: u64, // bitfield https://docs.discord.com/developers/topics/permissions#role-object-role-flags
}

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object-role-colors-object
#[derive(Serialize, Deserialize)]
struct Color {
    primary_color: u32,
    secondary_color: Option<u32>,
    tertiary_color: Option<u32>,
}

pub fn present<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    serde::de::IgnoredAny::deserialize(d)?;
    Ok(true)
}

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object-role-tags-structure
#[derive(Serialize, Deserialize)]
struct RoleTags {
    bot_id: Option<Snowflake>,
    integration_id: Option<Snowflake>,
    #[serde(deserialize_with = "present")]
    premium_subscriber: bool,
    subscription_listing_id: Option<Snowflake>,
    #[serde(deserialize_with = "present")]
    available_for_purchase: bool,
    #[serde(deserialize_with = "present")]
    guild_connections: bool,
}
