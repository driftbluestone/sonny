use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

const DISCORD_EPOCH: u64 = 1420070400000;

/**
 * These structs are near directly copied from the Discord developer docs.
 * They should be in a form such that payloads from GET requests can be
 * directly transformed into these structs. They are rather awkward to
 * use, though. So at some point we will want to write a converter to more
 * ergonomic structs, like Member objects in Discord.py.
 */

// note to self: make "flags" some sort of enum probably
// or hashmap of string: bool ?
// google "bit field serialization"

/* Snowflake */
/// Reference:  https://docs.discord.com/developers/reference#snowflakes
/// 
/// Implements to/from String and u64.
/// Can also be converted to timestamps.
#[derive(Serialize, Deserialize)]
pub struct Snowflake(String);

impl TryFrom<String> for Snowflake {
    type Error = std::num::ParseIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<u64>()?;
        Ok(Snowflake ( s ))
    }
}

impl From<u64> for Snowflake {
    fn from(i: u64) -> Self {
        Snowflake(i.to_string())
    }
}

impl From<Snowflake> for String {
    fn from(s: Snowflake) -> Self {
        s.0
    }
}

impl From<Snowflake> for u64 {
    fn from(s: Snowflake) -> Self {
        s.0.parse().unwrap()
    }
}

#[allow(unused)]
impl Snowflake {
    fn to_timestamp_ms(self) -> u64 {
        let n: u64 = self.into();
        (n >> 22) + DISCORD_EPOCH
    }

    fn to_timestamp_s(self) -> u64 {
        self.to_timestamp_ms() / 1000
    }
}

/* User */
/// Reference: https://docs.discord.com/developers/resources/user
#[derive(Serialize, Deserialize)]
struct UserBase {
    id: Snowflake,
    username: String,
    global_name: Option<String>,
    avatar: Option<String>,
    bot: bool,
    system: bool,
    mfa_enabled: bool,
    banner: Option<String>,
    accent_color: Option<u32>,
    locale: String,
    verified: bool,
    email: Option<String>,
    flags: u32, // bitfield: https://docs.discord.com/developers/resources/user#user-object-user-flags
    premium_type: PremiumType, // bitfield: https://docs.discord.com/developers/resources/user#user-object-premium-types
    public_flags: u32, // bitfield
    avatar_decoration_data: AvatarDecorationData,
    collectibles: Collectible, 
    primary_guild: UserPrimaryGuild,
}

/* Member */
/// Reference: https://docs.discord.com/developers/resources/guild#guild-member-object
/// 
/// I think it can only be acquirred in this form from GET /guilds/{guild_id}/members/{member_id}
#[derive(Serialize, Deserialize)]
struct MemberBase {
    user: Option<UserBase>, // wont be included in member object in message create/update events
    nick: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    roles: Vec<Snowflake>,
    #[serde(with = "time::serde::iso8601::option")]
    joined_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    premium_since: Option<OffsetDateTime>,
    deaf: bool,
    mute: bool,
    flags: u32, // bitfield
    pending: bool,
    permissions: String,
    #[serde(with = "time::serde::iso8601::option")]
    communication_disabled_until: Option<OffsetDateTime>,
    avatar_decoration_data: AvatarDecorationData,
    collectibles: Collectible,
}

/// Reference: https://docs.discord.com/developers/resources/guild
#[derive(Serialize, Deserialize)]
struct GuildBase {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: bool,
    owner_id: Snowflake,
    permissions: String,
    afk_channel_id: Option<Snowflake>,
    afk_timeout: u64,
    widget_enabled: bool,
    widget_channel_id: Option<Snowflake>,
    default_message_notifications: DefaultMessageNotifications,
    explicit_content_filter: ExplicitContentFilterLevel,
    mfa_level: MFALevel,
    verification_level: VerificationLevel,
    nsfw_level: GuildAgeRestrictionLevel,
    premium_tier: PremiumTier,
    roles: Vec<RoleBase>,
    emojis: Vec<EmojiBase>,
    features: Vec<String>, // TBI (technically): https://docs.discord.com/developers/resources/guild#guild-object-guild-features
    application_id: Option<Snowflake>,
    system_channel_id: Option<Snowflake>,
    system_channel_flags: u32, // bitfield https://docs.discord.com/developers/resources/guild#guild-object-system-channel-flags
    rules_channel_id: Option<Snowflake>,
    max_presences: Option<u64>,
    max_members: u64,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_subscription_count: u64, // number of boosts in the server
    preferred_locale: String,
    public_updates_channel_id: Option<Snowflake>,
    max_video_channel_users: u64,
    max_stage_video_channel_users: u64,
    approximate_member_count: u64,
    appriximate_presence_count: u64,
    welcome_screen: (), // TBI: https://docs.discord.com/developers/resources/guild#welcome-screen-object
    stickers: Vec<()>, // TBI: https://docs.discord.com/developers/resources/sticker#sticker-object
    premium_progress_bar_enabled: bool,
    safety_alerts_channel_id: Option<Snowflake>,
    incidents_data: Option<()>, // TBI: https://docs.discord.com/developers/resources/guild#incidents-data-object
}

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object
#[derive(Serialize, Deserialize)]
struct RoleBase {
    id: Snowflake,
    name: String,
    colors: (), // TBI: https://docs.discord.com/developers/topics/permissions#role-object-role-colors-object
    hoist: bool,
    icon: Option<String>,
    unicode_emoji: Option<String>,
    position: u64,
    permissions: String, // apparently it's a "permission bit set", look into that later
    managed: bool,
    mentionable: bool,
    tags: (), // TBI: https://docs.discord.com/developers/topics/permissions#role-object-role-tags-structure
    flags: u64 // bitfield https://docs.discord.com/developers/topics/permissions#role-object-role-flags
}

/// Reference: https://docs.discord.com/developers/resources/emoji#emoji-object
#[derive(Serialize, Deserialize)]
struct EmojiBase {
    id: Option<Snowflake>,
    name: Option<String>,
    roles: Vec<Snowflake>, // roles allowed to use thsi emoji
    user: UserBase, // user that created this emoji
    require_colons: bool,
    managed: bool,
    animated: bool,
    available: bool,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum DefaultMessageNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum ExplicitContentFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MFALevel {
    None = 0,
    Elevated = 1,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum GuildAgeRestrictionLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3, 
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3
}

/// Reference: https://docs.discord.com/developers/resources/user#user-object-user-primary-guild
#[derive(Serialize, Deserialize)]
struct UserPrimaryGuild {
    identity_guild_id: Option<Snowflake>,
    identity_enabled: Option<bool>,
    tag: Option<String>,
    badge: Option<String>,
}

/// Reference: https://docs.discord.com/developers/resources/user#avatar-decoration-data-object
#[derive(Serialize, Deserialize)]
struct AvatarDecorationData {
    asset: String,
    sku_id: Snowflake,
}

/// Reference: https://docs.discord.com/developers/resources/user#collectibles
#[derive(Serialize, Deserialize)]
struct Collectible {
    nameplate: Nameplate,
}

/// Reference: https://docs.discord.com/developers/resources/user#nameplate
#[derive(Serialize, Deserialize)]
struct Nameplate {
    sku_id: Snowflake,
    asset: String,
    label: String,
    palette: String,
}