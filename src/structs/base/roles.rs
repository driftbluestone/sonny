use crate::structs::base::snowflake::*;

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object
#[derive(Serialize, Deserialize)]
pub struct RoleBase {
    id: Snowflake,
    name: String,
    colors: (), // TBI: https://docs.discord.com/developers/topics/permissions#role-object-role-colors-object
    hoist: bool,
    icon: NullableOption<String>,
    unicode_emoji: NullableOption<String>,
    position: u64,
    permissions: String, // apparently it's a "permission bit set", look into that later
    managed: bool,
    mentionable: bool,
    tags: Option<()>, // TBI: https://docs.discord.com/developers/topics/permissions#role-object-role-tags-structure
    flags: u64 // bitfield https://docs.discord.com/developers/topics/permissions#role-object-role-flags
}