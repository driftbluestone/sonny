use crate::structs::base::{channels::*, emotes::*, roles::*, snowflake::*};

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

/// Reference: https://docs.discord.com/developers/resources/user
#[derive(Serialize, Deserialize)]
pub struct UserBase {
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

/// Reference: https://docs.discord.com/developers/resources/guild#guild-member-object
/// 
/// I think it can only be acquirred in this form from GET /guilds/{guild_id}/members/{member_id}
#[derive(Serialize, Deserialize)]
pub struct MemberBase {
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
pub struct GuildBase {
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
    features: Vec<GuildFeature>,
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
    welcome_screen: Option<WelcomeScreen>,
    stickers: Option<Vec<Sticker>>,
    premium_progress_bar_enabled: bool,
    safety_alerts_channel_id: Nullable<Snowflake>,
    incidents_data: Nullable<IncidentsData>,
}

/// Reference: https://docs.discord.com/developers/resources/guild#incidents-data-object
#[derive(Serialize, Deserialize)]
pub struct IncidentsData {
    #[serde(with = "time::serde::iso8601::option")]
    invites_disabled_until: Nullable<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    dms_disabled_until: Nullable<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    dm_spam_detected_at: NullableOption<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    raid_detected_at: NullableOption<OffsetDateTime>,
}

/// Reference: https://docs.discord.com/developers/resources/guild#guild-object-guild-features
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum GuildFeature {
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    Banner,
    Community,
    CreatorMonetizableProvisional,
    CreatorStorePage,
    DeveloperSupportServer,
    Discoverable,
    EnhancedRoleColors,
    Featurable,
    GuildTags,
    GuestsEnabled,
    InvitesDisabled,
    InviteSplash,
    MemberVerificationGateEnabled,
    MoreSoundboard,
    MoreStickers,
    News,
    Partnered,
    PreviewEnabled,
    PruneRequiresAdmin,
    RaidAlertsDisabled,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    Soundboard,
    TicketedEventsEnabled,
    VanityUrl,
    Verified,
    VipRegions,
    WelcomeScreenEnabled
}

#[derive(Serialize, Deserialize)]
pub struct Overwrite {
    id: Snowflake,
    r#type: OverwriteType,
    allow: String, // actually permission bit set
    deny: String, // also permission bit set
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum OverwriteType {
    Role,
    Member,
}


#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum DefaultMessageNotifications {
    AllMessages,
    OnlyMentions,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum ExplicitContentFilterLevel {
    Disabled,
    MembersWithoutRoles,
    AllMembers,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MFALevel {
    None,
    Elevated,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum GuildAgeRestrictionLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PremiumTier {
    None,
    Tier1,
    Tier2,
    Tier3, 
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    NitroBasic
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

/// Reference: https://docs.discord.com/developers/resources/guild#welcome-screen-object
#[derive(Serialize, Deserialize)]
struct WelcomeScreen {
    description: Nullable<String>,
    welcome_channels: Vec<WelcomeScreenChannel>,
}

/// Reference: https://docs.discord.com/developers/resources/guild#welcome-screen-object-welcome-screen-channel-structure
#[derive(Serialize, Deserialize)]
struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Nullable<Snowflake>,
    emoji_name: Nullable<String>,
}