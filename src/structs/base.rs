use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

const DISCORD_EPOCH: u64 = 1420070400000;
type Nullable<T> = Option<T>;
type NullableOption<T> = Option<T>;

/**
 * These structs are near directly copied from the Discord developer docs.
 * They should be in a form such that payloads from GET requests can be
 * directly transformed into these structs. They are rather awkward to
 * use, though. So at some point we will want to write a converter to more
 * ergonomic structs, like Member objects in Discord.py.
 */

/**
 * Typing:
 *  Option:
 *   field? type
 *   field may not exist in the json
 *  Nullable:
 *   field ?type
 *   field will exist but may be null
 *  NullableOption:
 *   field? ?type
 *   field may not exist and may be null
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
    global_name: Nullable<String>,
    avatar: Nullable<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Nullable<String>,
    accent_color: Nullable<u32>,
    locale: Option<String>,
    verified: Option<bool>,
    email: NullableOption<String>,
    flags: Option<u32>, // bitfield: https://docs.discord.com/developers/resources/user#user-object-user-flags
    premium_type: Option<PremiumType>, // bitfield: https://docs.discord.com/developers/resources/user#user-object-premium-types
    public_flags: Option<u32>, // bitfield
    avatar_decoration_data: Option<AvatarDecorationData>,
    collectibles: NullableOption<Collectible>, 
    primary_guild: Option<UserPrimaryGuild>,
}

/* Member */
/// Reference: https://docs.discord.com/developers/resources/guild#guild-member-object
/// 
/// I think it can only be acquirred in this form from GET /guilds/{guild_id}/members/{member_id}
#[derive(Serialize, Deserialize)]
struct MemberBase {
    user: Option<UserBase>, // wont be included in member object in message create/update events
    nick: NullableOption<String>,
    avatar: NullableOption<String>,
    banner: NullableOption<String>,
    roles: Vec<Snowflake>,
    #[serde(with = "time::serde::iso8601::option")]
    joined_at: Nullable<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    premium_since: NullableOption<OffsetDateTime>,
    deaf: bool,
    mute: bool,
    flags: u32, // bitfield
    pending: Option<bool>,
    permissions: Option<String>,
    #[serde(with = "time::serde::iso8601::option")]
    communication_disabled_until: NullableOption<OffsetDateTime>,
    avatar_decoration_data: NullableOption<AvatarDecorationData>,
    collectibles: NullableOption<Collectible>,
}

/// Reference: https://docs.discord.com/developers/resources/guild
#[derive(Serialize, Deserialize)]
struct GuildBase {
    id: Snowflake,
    name: String,
    icon: Nullable<String>,
    icon_hash: NullableOption<String>,
    splash: Nullable<String>,
    discovery_splash: Nullable<String>,
    owner: Option<bool>,
    owner_id: Snowflake,
    permissions: Option<String>,
    afk_channel_id: Nullable<Snowflake>,
    afk_timeout: u64,
    widget_enabled: Option<bool>,
    widget_channel_id: NullableOption<Snowflake>,
    default_message_notifications: DefaultMessageNotifications,
    explicit_content_filter: ExplicitContentFilterLevel,
    mfa_level: MFALevel,
    verification_level: VerificationLevel,
    nsfw_level: GuildAgeRestrictionLevel,
    premium_tier: PremiumTier,
    roles: Vec<RoleBase>,
    emojis: Vec<EmojiBase>,
    features: Vec<String>, // TBI (technically): https://docs.discord.com/developers/resources/guild#guild-object-guild-features
    application_id: Nullable<Snowflake>,
    system_channel_id: Nullable<Snowflake>,
    system_channel_flags: u32, // bitfield https://docs.discord.com/developers/resources/guild#guild-object-system-channel-flags
    rules_channel_id: Nullable<Snowflake>,
    max_presences: NullableOption<u64>,
    max_members: Option<u64>,
    vanity_url_code: Nullable<String>,
    description: Nullable<String>,
    banner: Nullable<String>,
    premium_subscription_count: Option<u64>, // number of boosts in the server
    preferred_locale: String,
    public_updates_channel_id: Nullable<Snowflake>,
    max_video_channel_users: Option<u64>,
    max_stage_video_channel_users: Option<u64>,
    approximate_member_count: Option<u64>,
    appriximate_presence_count: Option<u64>,
    welcome_screen: Option<()>, // TBI: https://docs.discord.com/developers/resources/guild#welcome-screen-object
    stickers: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/sticker#sticker-object
    premium_progress_bar_enabled: bool,
    safety_alerts_channel_id: Nullable<Snowflake>,
    incidents_data: Nullable<()>, // TBI: https://docs.discord.com/developers/resources/guild#incidents-data-object
}

/// Reference: https://docs.discord.com/developers/resources/message#message-object
#[derive(Serialize, Deserialize)]
struct MessageBase {
    id: Snowflake,
    channel_id: Snowflake,
    author: UserBase,
    content: String,
    #[serde(with = "time::serde::iso8601")]
    timestamp: OffsetDateTime,
    #[serde(with = "time::serde::iso8601::option")]
    edited_timestamp: Nullable<OffsetDateTime>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<UserBase>,
    mention_roles: Option<Vec<Snowflake>>,
    mention_channels: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/message#channel-mention-object
    attatchments: Vec<()>, // TBI: https://docs.discord.com/developers/resources/message#attachment-object
    embeds: Vec<()>, // TBI: https://docs.discord.com/developers/resources/message#embed-object
    reactions: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/message#reaction-object
    pinned: bool,
    webhook_id: Option<Snowflake>,
    r#type: u8, // TBI enum: https://docs.discord.com/developers/resources/message#message-object-message-types
    activity: (), // TBI: https://docs.discord.com/developers/resources/message#message-object-message-activity-structure
    application: (), // (partial) TBI: https://docs.discord.com/developers/resources/application#application-object
    application_id: Snowflake,
    flags: u64, // bitfield https://docs.discord.com/developers/resources/message#message-object-message-flags
    message_reference: Option<()>, // TBI: https://docs.discord.com/developers/resources/message#message-reference-structure
    message_snapshots: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/message#message-snapshot-object
    // referenced_message: NullableOption<MessageBase>, // this will have to be Dealt With
    interaction_metadata: Option<()>, // TBI: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object
    interaction: Option<()>, // TBI: https://docs.discord.com/developers/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
    thread: Option<()>, // TBI: https://docs.discord.com/developers/resources/channel#channel-object
    components: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/components/reference#component-object
    sticker_items: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/sticker#sticker-item-object
    stickers: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/resources/sticker#sticker-object
    position: Option<i64>,
    role_subscription_data: Option<()>, // TBI: https://docs.discord.com/developers/resources/message#role-subscription-data-object
    resolved: Option<()>, // TBI: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-resolved-data-structure
    poll: Option<()>, // TBI: https://docs.discord.com/developers/resources/poll#poll-object
    call: Option<()>, // TBI: https://docs.discord.com/developers/resources/message#message-call-object
    shared_client_theme: Option<()>, // TBI: https://docs.discord.com/developers/resources/message#shared-client-theme-object
}

/// Reference: https://docs.discord.com/developers/topics/permissions#role-object
#[derive(Serialize, Deserialize)]
struct RoleBase {
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

/// Reference: https://docs.discord.com/developers/resources/emoji#emoji-object
#[derive(Serialize, Deserialize)]
struct EmojiBase {
    id: Nullable<Snowflake>,
    name: Nullable<String>,
    roles: Option<Vec<Snowflake>>, // roles allowed to use thsi emoji
    user: Option<UserBase>, // user that created this emoji
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
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
    identity_guild_id: Nullable<Snowflake>,
    identity_enabled: Nullable<bool>,
    tag: Nullable<String>,
    badge: Nullable<String>,
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
    nameplate: Option<Nameplate>,
}

/// Reference: https://docs.discord.com/developers/resources/user#nameplate
#[derive(Serialize, Deserialize)]
struct Nameplate {
    sku_id: Snowflake,
    asset: String,
    label: String,
    palette: String,
}