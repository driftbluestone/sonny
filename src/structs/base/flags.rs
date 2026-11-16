//! Probably shouldn't actually be in base, but
//! this is meant to implement more ergonomic
//! forms of bitfield values in various structs.

/// Reference: https://docs.discord.com/developers/resources/user#user-object-user-flags
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserFlags {
    /* bits 0 ~ 3 */
    staff: bool,
    partner: bool,
    hypesquad: bool,
    bug_hunter_level_1: bool,
    /* bits 6 ~ 10 */
    hypesquad_online_house_1: bool,
    hypesquad_online_house_2: bool,
    hypesquad_online_house_3: bool,
    premium_early_supporter: bool,
    team_pseudo_user: bool, // https://docs.discord.com/developers/topics/teams
    /* bit 14 */
    bug_hunter_level_2: bool,
    /* bits 16 ~ 19 */
    verified_bot: bool,
    verified_developer: bool,
    certified_moderator: bool,
    bot_http_interactions: bool,
}

// impl UserFlags {pub fn new(v: u64) -> Self {}}
// this will be Interesting to implement.

/// Reference: https://docs.discord.com/developers/resources/guild#guild-object-guild-features
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GuildFeatures {
    animated_banner: bool,
    animated_icon: bool,
    application_command_permissions_v2: bool,
    auto_moderation: bool,
    banner: bool,
    community: bool,
    creator_monetizable_provisional: bool,
    creator_store_page: bool,
    developer_support_server: bool,
    discoverable: bool,
    enhanced_role_colors: bool,
    featurable: bool,
    guild_tags: bool,
    guests_enabled: bool,
    invites_disabled: bool,
    invite_splash: bool,
    member_verification_gate_enabled: bool,
    more_soundboard: bool,
    more_stickers: bool,
    news: bool,
    partnered: bool,
    preview_enabled: bool,
    prune_requires_admin: bool,
    raid_alerts_disabled: bool,
    role_icons: bool,
    role_subscriptions_available_for_purchase: bool,
    role_subscriptions_enabled: bool,
    soundboard: bool,
    ticketed_events_enabled: bool,
    vanity_url: bool,
    verified: bool,
    vip_regions: bool
}

impl GuildFeatures {
    pub fn new(features: Vec<String>) -> Self {
        let mut map = serde_json::Map::new();
        for feature in features {
            map.insert(feature, serde_json::Value::Bool(true));
        }
        serde_json::from_value(serde_json::Value::Object(map)).unwrap_or_default()
    }
}