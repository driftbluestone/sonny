use std::collections::HashMap;

use crate::structs::base::{
    application::{Application, IntegrationType},
    attachment::Attachment,
    base::{MemberBase, UserBase},
    channels::*,
    embed::Embed,
    emotes::*,
    roles::RoleBase,
    snowflake::*,
};

/// Reference: https://docs.discord.com/developers/resources/message#message-object
#[derive(Serialize, Deserialize)]
pub struct MessageBase {
    id: Snowflake,
    channel_id: Snowflake,
    author: UserBase,
    content: String,
    #[serde(with = "time::serde::iso8601")]
    timestamp: OffsetDateTime,
    #[serde(with = "time::serde::iso8601::option")]
    edited_timestamp: Nullable<OffsetDateTime>,
    tts: bool, // is text to speech
    mention_everyone: bool,
    mentions: Vec<UserBase>,
    mention_roles: Vec<Snowflake>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<Nonce>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    r#type: MessageType,
    activity: Option<MessageActivity>,
    application: Option<Application>, // (partial)
    application_id: Option<Snowflake>,
    flags: Option<u64>, // bitfield https://docs.discord.com/developers/resources/message#message-object-message-flags
    message_reference: Option<MessageReference>,
    message_snapshots: Option<Vec<MessageSnapshot>>,
    referenced_message: NullableOption<Box<MessageBase>>,
    interaction_metadata: Option<InteractionMetadata>,
    interaction: Option<Interaction>,
    thread: Option<Channel>,
    components: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/components/reference#component-object
    sticker_items: Option<Vec<StickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<i64>,
    role_subscription_data: Option<RoleSubscriptionData>,
    resolved: Option<Resolved>,
    poll: Option<Poll>,
    call: Option<Call>,
    shared_client_theme: Option<SharedClientTheme>,
}

/// Reference: https://docs.discord.com/developers/resources/message#channel-mention-object
#[derive(Serialize, Deserialize)]
struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    r#type: ChannelType,
    name: String,
}

/// Reference: https://docs.discord.com/developers/resources/message#reaction-object
#[derive(Serialize, Deserialize)]
struct Reaction {
    count: u32,
    count_details: ReactionCountDetails,
    me: bool,
    me_burst: bool,
    emoji: EmojiBase, // (partial)
    burst_colors: Vec<String>,
}

/// Reference: https://docs.discord.com/developers/resources/message#reaction-count-details-object
#[derive(Serialize, Deserialize)]
struct ReactionCountDetails {
    burst: u32,
    normal: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Nonce {
    Int(u64),
    Str(String),
}

/// Reference: https://docs.discord.com/developers/resources/message#message-object-message-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-object-message-activity-structure
#[derive(Serialize, Deserialize)]
struct MessageActivity {
    r#type: MessageActivityType,
    party_id: Option<String>,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-object-message-activity-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
    StreamRequest = 6,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-reference-structure
#[derive(Serialize, Deserialize)]
struct MessageReference {
    r#type: Option<MessageReferenceType>,
    message_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    guild_id: Option<Snowflake>,
    fail_if_not_exists: Option<bool>,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-reference-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MessageReferenceType {
    Default = 0,
    Forward = 1,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-snapshot-object
#[derive(Serialize, Deserialize)]
struct MessageSnapshot {
    message: MessageBase, // (partial)
}

/// Reference: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object
#[derive(Serialize, Deserialize)]
struct InteractionMetadata {
    id: Snowflake,
    r#type: InteractionType,
    user: UserBase,
    authorizing_integration_owners: HashMap<IntegrationType, Snowflake>,
    original_response_message_id: Option<Snowflake>,
    target_user: Option<UserBase>,
    target_message_id: Option<Snowflake>,
    interacted_message_id: Option<Snowflake>,
    triggering_interaction_metadata: Option<Box<InteractionMetadata>>,
}

/// Reference: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

/// Reference: https://docs.discord.com/developers/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
#[derive(Serialize, Deserialize)]
struct Interaction {
    id: Snowflake,
    r#type: InteractionType,
    name: String,
    user: UserBase,
    member: Option<MemberBase>, // (partial)
}

/// Reference: https://docs.discord.com/developers/resources/message#role-subscription-data-object
#[derive(Serialize, Deserialize)]
struct RoleSubscriptionData {
    role_subscription_listing_id: Snowflake,
    tier_name: String,
    total_months_subscribed: u32,
    is_renewal: bool,
}

/// Reference: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Serialize, Deserialize)]
struct Resolved {
    users: Option<HashMap<Snowflake, UserBase>>,
    members: Option<HashMap<Snowflake, MemberBase>>, // (partial) member
    roles: Option<HashMap<Snowflake, RoleBase>>,
    channels: Option<HashMap<Snowflake, Channel>>, // (partial) channel
    messages: Option<HashMap<Snowflake, MessageBase>>, // (partial) message
    attachments: Option<HashMap<Snowflake, Attachment>>,
}

/// Reference: https://docs.discord.com/developers/resources/poll#poll-object
#[derive(Serialize, Deserialize)]
struct Poll {
    question: PollMedia,
    answers: Vec<PollAnswer>,
    #[serde(with = "time::serde::iso8601::option")]
    expiry: Nullable<OffsetDateTime>,
    allow_multiselect: bool,
    layout_type: Option<PollLayoutType>,
    results: Option<PollResults>,
}

/// Reference: https://docs.discord.com/developers/resources/poll#poll-media-object-poll-media-object-structure
#[derive(Serialize, Deserialize)]
struct PollMedia {
    text: Option<String>,
    emoji: Option<EmojiBase>, // (partial)
}

/// Reference: https://docs.discord.com/developers/resources/poll#poll-answer-object-poll-answer-object-structure
#[derive(Serialize, Deserialize)]
struct PollAnswer {
    answer_id: u32,
    poll_media: PollMedia,
}

/// Reference: https://docs.discord.com/developers/resources/poll#layout-type
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PollLayoutType {
    Default = 1,
}

/// Reference: https://docs.discord.com/developers/resources/poll#poll-results-object-poll-results-object-structure
#[derive(Serialize, Deserialize)]
struct PollResults {
    is_finalized: bool,
    answer_counts: Vec<PollAnswerCount>,
}

/// Reference: https://docs.discord.com/developers/resources/poll#poll-results-object-poll-answer-count-object-structure
#[derive(Serialize, Deserialize)]
struct PollAnswerCount {
    id: u32,
    count: u32,
    me_voted: bool,
}

/// Reference: https://docs.discord.com/developers/resources/message#message-call-object
#[derive(Serialize, Deserialize)]
struct Call {
    participants: Vec<Snowflake>,
    #[serde(with = "time::serde::iso8601::option")]
    ended_timestamp: NullableOption<OffsetDateTime>,
}

/// Reference: https://docs.discord.com/developers/resources/message#shared-client-theme-object
#[derive(Serialize, Deserialize)]
struct SharedClientTheme {
    colors: Vec<String>,
    gradient_angle: u16,
    base_mix: u8,
    base_theme: NullableOption<BaseThemeType>,
}

/// Reference: https://docs.discord.com/developers/resources/message#base-theme-types
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum BaseThemeType {
    Unset = 0,
    Dark = 1,
    Light = 2,
    Darker = 3,
    Midnight = 4,
}
