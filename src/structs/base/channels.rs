use crate::structs::base::{base::{MemberBase, Overwrite, UserBase}, snowflake::*};

/// Reference: https://docs.discord.com/developers/resources/channel
#[derive(Serialize, Deserialize)]
pub struct Channel {
    id: Snowflake,
    r#type: ChannelType,
    guild_id: Option<Snowflake>,
    position: Option<i64>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: NullableOption<String>,
    topic: NullableOption<String>,
    nsfw: Option<bool>,
    last_message_id: NullableOption<Snowflake>,
    bitrate: Option<u64>, // bitrate (bits/sec) of the voice channel
    user_limit: Option<u64>, // user limit of the voice channel
    rate_limit_per_user: Option<u64>, // slowmode (seconds)
    recipients: Option<Vec<UserBase>>, // recipients of the dm
    icon: NullableOption<String>,
    owner_id: Option<Snowflake>, // create of group dm or thread
    managed: Option<bool>,
    parent_id: NullableOption<Snowflake>, // parent category of channel or channel thread was created in
    #[serde(with = "time::serde::iso8601::option")]
    last_pin_timestamp: NullableOption<OffsetDateTime>,
    rtc_region: NullableOption<String>,
    video_quality_mode: Option<VideoQualityMode>,
    message_count: Option<u64>, // num messages in a thread
    member_count: Option<u64>, // num members in a thread, stops counting after 50.
    thread_metadata: Option<ThreadMetaData>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: Option<u16>,
    permissions: Option<String>,
    app_permissions: Option<String>,
    flags: u64, // bitfield https://docs.discord.com/developers/resources/channel#channel-object-channel-flags
    total_message_sent: Option<u64>, // similar to message count, but isn't decremented on message delete
    available_tags: Option<Vec<ForumTag>>,
    applied_tags: Option<Vec<Snowflake>>,
    default_reaction_emoji: NullableOption<DefaultReaction>,
    default_thread_rate_limit_per_user: Option<u64>,
    default_sort_order: NullableOption<SortOrderType>,
    default_forum_layout: Option<ForumLayoutType>
}

/// Reference: https://docs.discord.com/developers/resources/channel#default-reaction-object
#[derive(Serialize, Deserialize)]
struct DefaultReaction {
    emoji_id: Nullable<Snowflake>,
    emoji_name: Nullable<String>,
}

/// Reference: https://docs.discord.com/developers/resources/channel#forum-tag-object
#[derive(Serialize, Deserialize)]
struct ForumTag {
    id: Snowflake,
    name: String,
    moderated: bool,
    // at most one of these may be null
    emoji_id: Nullable<Snowflake>,
    emoji_name: Nullable<String>,
}

/// Reference: https://docs.discord.com/developers/resources/channel#thread-metadata-object
#[derive(Serialize, Deserialize)]
struct ThreadMetaData {
    archived: bool,
    auto_archive_duration: u16,
    #[serde(with = "time::serde::iso8601")]
    archive_timstamp: OffsetDateTime,
    locked: bool,
    invitable: Option<bool>,
    #[serde(with = "time::serde::iso8601::option")]
    create_timestamp: NullableOption<OffsetDateTime>,
}

/// Reference https://docs.discord.com/developers/resources/channel#thread-member-object
#[derive(Serialize, Deserialize)]
struct ThreadMember {
    id: Option<Snowflake>, // thread id
    user_id: Option<Snowflake>, // user id
    #[serde(with = "time::serde::iso8601")]
    join_timestamp: OffsetDateTime,
    flags: u64, // bitfield?
    member: Option<MemberBase>,
}

/// Reference: https://docs.discord.com/developers/resources/channel#channel-object-channel-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum ChannelType {
    GuildText,
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
    GuildMedia, // still in active development
}

/// Reference: https://docs.discord.com/developers/resources/channel#channel-object-video-quality-modes
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum VideoQualityMode {
    Auto = 1,
    Full,
}

/// Reference: https://docs.discord.com/developers/resources/channel#channel-object-sort-order-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum SortOrderType {
    LatestActivity,
    CreationDate,
}

/// Reference: https://docs.discord.com/developers/resources/channel#channel-object-forum-layout-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum ForumLayoutType {
    NotSet,
    ListView,
    GalleryView,
}