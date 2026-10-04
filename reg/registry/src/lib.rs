use std::{any::Any, collections::HashMap, future::Future, pin::Pin, sync::{LazyLock, Mutex}};
use phf::phf_map;
use tokio;
use serenity::all::{CreateCommand, ShardStageUpdateEvent};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub static EVENT_REG: LazyLock<Mutex<HashMap<&'static str, Vec<fn(Vec<Box<dyn Any + Send + Sync>>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub static CMD_REG: LazyLock<Mutex<HashMap<&'static str, fn(Vec<Box<dyn Any + Send + Sync>>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub static ACMD_REG: LazyLock<Mutex<HashMap<&'static str, fn(Vec<Box<dyn Any + Send + Sync>>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub static ACMD_TYPE_REG: LazyLock<Mutex<HashMap<&'static str, Vec<(String, String, bool)>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub static TO_REG: LazyLock<Mutex<Vec<CreateCommand>>> = LazyLock::new(|| {Mutex::new(Vec::new())});

pub fn register_event(
    key: &'static str, 
    f: fn(Vec<Box<dyn Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>
) {
    if let Ok(mut registry) = EVENT_REG.lock() {
        registry.entry(key).or_insert_with(Vec::new).push(f);
    }
}

pub fn register_cmd(
    key: &'static str, 
    f: fn(Vec<Box<dyn Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>
) {
    if let Ok(mut registry) = CMD_REG.lock() {
        registry.insert(key, f);
    }
}

pub fn register_acmd(
    key: &'static str,
    f: fn(Vec<Box<dyn Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>
) {
    if let Ok(mut registry) = ACMD_REG.lock() {
        registry.insert(key, f);
    }
}

pub fn register_acmd_create(create_cmd: CreateCommand, key: &'static str, arg_types: Vec<(String, String, bool)>) {
    if let Ok(mut registry) = TO_REG.lock() {
        registry.push(create_cmd);
    }
    if let Ok(mut registry) = ACMD_TYPE_REG.lock() {
        registry.insert(key, arg_types);
    }
}

#[macro_export]
macro_rules! run_event {
    ($event_type:literal, $($args:expr),*) => {{
        if let Ok(registry) = EVENT_REG.lock() {
            if let Some(callbacks) = registry.get($event_type) {
                for callback in callbacks {
                    let mut callback_args: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
                    $(
                        callback_args.push(Box::new($args.clone()) as Box<dyn std::any::Any + Send + Sync>);
                    )*

                    tokio::spawn(callback(callback_args));
                }
            }
        }
    }};
}

pub const ARG_TYPE: phf::Map<&'static str, CommandOptionType> = phf_map! {
    // String Types
    "&str" => CommandOptionType::String,
    "String" => CommandOptionType::String,
    /* Numeric Types */
    // Signed Ints
    "i8" => CommandOptionType::Integer,
    "i16" => CommandOptionType::Integer,
    "i32" => CommandOptionType::Integer,
    "i64" => CommandOptionType::Integer,
    "i128" => CommandOptionType::Integer,
    // Unsigned Ints
    "u8" => CommandOptionType::Integer,
    "u16" => CommandOptionType::Integer,
    "u32" => CommandOptionType::Integer,
    "u64" => CommandOptionType::Integer,
    "u128" => CommandOptionType::Integer,
    // Floats
    "f16" => CommandOptionType::Number,
    "f32" => CommandOptionType::Number,
    "f64" => CommandOptionType::Number,
    "f128" => CommandOptionType::Number,
    // Boolean
    "bool" => CommandOptionType::Boolean,
    /* Discord Types */
    // User Types
    "User" => CommandOptionType::User,
    "UserId" => CommandOptionType::User,
    "Member" => CommandOptionType::User,
    // Channels
    "Channel" => CommandOptionType::Channel,
    "ChannelId" => CommandOptionType::Channel,
    // Mentionable
    "Role" => CommandOptionType::Role,
    "RoleId" => CommandOptionType::Role,
    "Mentionable" => CommandOptionType::Mentionable,
    // Other
    "AttatchmentId" => CommandOptionType::Attachment
};

/* The Cursed Zone */
pub struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    /* Macro rules for every event */
    async fn command_permissions_update(&self, ctx: Context, permission: CommandPermissions) {run_event!("command_permissions_update", ctx, permission);}
    async fn auto_moderation_rule_create(&self, ctx: Context, rule: Rule) {run_event!("auto_moderation_rule_create", ctx, rule);}
    async fn auto_moderation_rule_update(&self, ctx: Context, rule: Rule) {run_event!("auto_moderation_rule_update", ctx, rule);}
    async fn auto_moderation_rule_delete(&self, ctx: Context, rule: Rule) {run_event!("auto_moderation_rule_delete", ctx, rule);}
    async fn auto_moderation_action_execution(&self, ctx: Context, execution: ActionExecution) {run_event!("auto_moderation_action_execution", ctx, execution);}
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {run_event!("cache_ready", ctx, guilds);}
    async fn shards_ready(&self, ctx: Context, total_shards: u32) {run_event!("shards_ready", ctx, total_shards);}
    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {run_event!("channel_create", ctx, channel);}
    async fn category_create(&self, ctx: Context, category: GuildChannel) {run_event!("category_create", ctx, category);}
    async fn category_delete(&self, ctx: Context, category: GuildChannel) {run_event!("category_delete", ctx, category);}
    async fn channel_delete(&self, ctx: Context, channel: GuildChannel, messages: Option<Vec<Message>>) {run_event!("channel_create", ctx, channel, messages);}
    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {run_event!("channel_pins_update", ctx, pin);}
    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {run_event!("channel_update", ctx, old, new);}
    async fn guild_audit_log_entry_create(&self, ctx: Context, entry: AuditLogEntry, guild_id: GuildId) {run_event!("guild_audit_log_entry_create", ctx, entry, guild_id);}
    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {run_event!("guild_ban_addition", ctx, guild_id, banned_user);}
    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {run_event!("guild_ban_removal", ctx, guild_id, unbanned_user);}
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {run_event!("guild_create", ctx, guild, is_new);}
    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {run_event!("guild_delete", ctx, incomplete, full);}
    async fn guild_emojis_update(&self, ctx: Context, guild_id: GuildId, current_state: HashMap<EmojiId, Emoji>) {run_event!("guild_emojis_update", ctx, guild_id, current_state);}
    async fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {run_event!("guild_integrations_update", ctx, guild_id);}
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {run_event!("guild_member_addition", ctx, new_member);}
    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, member_data_if_available: Option<Member>) {run_event!("guild_member_removal", ctx, guild_id, user, member_data_if_available);}
    async fn guild_member_update(&self, ctx: Context, old_if_available: Option<Member>, new: Option<Member>, event: GuildMemberUpdateEvent) {run_event!("guild_member_update", ctx, old_if_available, new, event);}
    async fn guild_members_chunk(&self, ctx: Context, chunk: GuildMembersChunkEvent) {run_event!("guild_members_chunk", ctx, chunk);}
    async fn guild_role_create(&self, ctx: Context, new: Role) {run_event!("guild_role_create", ctx, new);}
    async fn guild_role_delete(&self, ctx: Context, guild_id: GuildId, removed_role_id: RoleId, removed_role_date_if_available: Option<Role>) {run_event!("guild_role_delete", ctx, guild_id, removed_role_id, removed_role_date_if_available);}
    async fn guild_role_update(&self, ctx: Context, old_data_if_available: Option<Role>, new: Role) {run_event!("guild_role_update", ctx, old_data_if_available, new);}
    async fn guild_stickers_update(&self, ctx: Context, guild_id: GuildId, current_state: HashMap<StickerId, Sticker>) {run_event!("guild_stickers_update", ctx, guild_id, current_state);}
    async fn guild_update(&self, ctx: Context, old_data_if_available: Option<Guild>, new_data: PartialGuild) {run_event!("guild_update", ctx, old_data_if_available, new_data);}
    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {run_event!("invite_create", ctx, data);}
    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {run_event!("invite_delete", ctx, data);}
    async fn message(&self, ctx: Context, new_message: Message) {run_event!("message", ctx, new_message);}
    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {run_event!("message_delete", ctx, channel_id, deleted_message_id, guild_id);}
    async fn message_delete_bulk(&self, ctx: Context, channel_id: ChannelId, multiple_deleted_messages_ids: Vec<MessageId>, guild_id: Option<GuildId>) {run_event!("message_delete_bulk", ctx, channel_id, multiple_deleted_messages_ids, guild_id);}
    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {run_event!("message_update", ctx, old_if_available, new, event);}
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {run_event!("reaction_add", ctx, add_reaction);}
    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {run_event!("reaction_remove", ctx, removed_reaction);}
    async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, removed_from_message_id: MessageId) {run_event!("reaction_remove_all", ctx, channel_id, removed_from_message_id);}
    async fn reaction_remove_emoji(&self, ctx: Context, removed_reactions: Reaction) {run_event!("reaction_remove_emoji", ctx, removed_reactions);}
    async fn presence_update(&self, ctx: Context, new_data: Presence) {run_event!("presence_update", ctx, new_data);}
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {run_event!("ready", ctx, data_about_bot);}
    async fn resume(&self, ctx: Context, event: ResumedEvent) {run_event!("resume", ctx, event);}
    async fn shard_stage_update(&self, ctx: Context, event: ShardStageUpdateEvent) {run_event!("shard_state_update", ctx, event);}
    async fn soundboard_sounds(&self, ctx: Context, event: SoundboardSoundsEvent) {run_event!("soundboard_sounds", ctx, event);}
    async fn soundboard_sound_create(&self, ctx: Context, event: SoundboardSoundCreateEvent) {run_event!("soundboard_sound_create", ctx, event);}
    async fn soundboard_sound_update(&self, ctx: Context, event: SoundboardSoundUpdateEvent) {run_event!("soundboard_sound_update", ctx, event);}
    async fn soundboard_sounds_update(&self, ctx: Context, event: SoundboardSoundsUpdateEvent) {run_event!("soundboard_sounds_update", ctx, event);}
    async fn soundboard_sound_delete(&self, ctx: Context, event: SoundboardSoundDeleteEvent) {run_event!("soundboard_sound_delete", ctx, event);}
    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {run_event!("typing_start", ctx, event);}
    async fn user_update(&self, ctx: Context, old_data: Option<CurrentUser>, new: CurrentUser) {run_event!("user_update", ctx, old_data, new);}
    async fn voice_server_update(&self, ctx: Context, event: VoiceServerUpdateEvent) {run_event!("voice_server_update", ctx, event);}
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {run_event!("voice_state_update", ctx, old, new);}
    async fn voice_channel_status_update(&self, ctx: Context, old: Option<String>, status: Option<String>, id: ChannelId, guild_id: GuildId) {run_event!("voice_channel_status_update", ctx, old, status, id, guild_id);}
    async fn webhook_update(&self, ctx: Context, guild_id: GuildId, belongs_to_channnel_id: ChannelId) {run_event!("webhook_update", ctx, guild_id, belongs_to_channnel_id);}
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {run_event!("interaction_create", ctx, interaction);}
    async fn integration_create(&self, ctx: Context, integration: Integration) {run_event!("integration_create", ctx, integration);}
    async fn integration_update(&self, ctx: Context, integration: Integration) {run_event!("integration_update", ctx, integration);}
    async fn integration_delete(&self, ctx: Context, integration_id: IntegrationId, guild_id: GuildId, application_id: Option<ApplicationId>) {run_event!("integration_delete", ctx, integration_id, guild_id, application_id);}
    async fn stage_instance_create(&self, ctx: Context, stage_instance: StageInstance) {run_event!("stage_instance_create", ctx, stage_instance);}
    async fn stage_instance_update(&self, ctx: Context, stage_instance: StageInstance) {run_event!("stage_instance_update", ctx, stage_instance);}
    async fn stage_instance_delete(&self, ctx: Context, stage_instance: StageInstance) {run_event!("stage_instance_delete", ctx, stage_instance);}
    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {run_event!("thread_create", ctx, thread);}
    async fn thread_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {run_event!("thread_update", ctx, old, new);}
    async fn thread_delete(&self, ctx: Context, thread: PartialGuildChannel, full_thread_data: Option<GuildChannel>) {run_event!("thread_delete", ctx, thread, full_thread_data);}
    async fn thread_list_sync(&self, ctx: Context, thread_list_sync: ThreadListSyncEvent) {run_event!("thread_list_sync", ctx, thread_list_sync);}
    async fn thread_member_update(&self, ctx: Context, thread_member: ThreadMember) {run_event!("thread_member_update", ctx, thread_member);}
    async fn thread_members_update(&self, ctx: Context, thread_members_update: ThreadMembersUpdateEvent) {run_event!("thread_members_update", ctx, thread_members_update);}
    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {run_event!("guild_scheduled_event_create", ctx, event);}
    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {run_event!("guild_scheduled_event_update", ctx, event);}
    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {run_event!("guild_scheduled_event_delete", ctx, event);}
    async fn guild_scheduled_event_user_add(&self, ctx: Context, subscribed: GuildScheduledEventUserAddEvent) {run_event!("guild_scheduled_event_user_add", ctx, subscribed);}
    async fn guild_scheduled_event_user_remove(&self, ctx: Context, unsubscribed: GuildScheduledEventUserRemoveEvent) {run_event!("guild_scheduled_event_user_remove", ctx, unsubscribed);}
    async fn entitlement_create(&self, ctx: Context, entitlement: Entitlement) {run_event!("entitlement_create", ctx, entitlement);}
    async fn entitlement_update(&self, ctx: Context, entitlement: Entitlement) {run_event!("entitlement_update", ctx, entitlement);}
    async fn entitlement_delete(&self, ctx: Context, entitlement: Entitlement) {run_event!("entitlement_delete", ctx, entitlement);}
    async fn poll_vote_add(&self, ctx: Context, event: MessagePollVoteAddEvent) {run_event!("poll_vote_add", ctx, event);}
    async fn poll_vote_remove(&self, ctx: Context, event: MessagePollVoteRemoveEvent) {run_event!("poll_vote_remove", ctx, event);}
}