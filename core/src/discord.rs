use std::collections::HashMap;
use serenity::all::ShardStageUpdateEvent;
use serenity::model::prelude::*;
use serenity::prelude::*;
use registry::REGISTRY;

macro_rules! run {
    ($event_type:literal, $($args:expr),*) => {{
        if let Ok(registry) = REGISTRY.lock() {
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

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    /* Macro rules for every event */
    async fn command_permissions_update(&self, ctx: Context, permission: CommandPermissions) {run!("command_permissions_update", ctx, permission);}
    async fn auto_moderation_rule_create(&self, ctx: Context, rule: Rule) {run!("auto_moderation_rule_create", ctx, rule);}
    async fn auto_moderation_rule_update(&self, ctx: Context, rule: Rule) {run!("auto_moderation_rule_update", ctx, rule);}
    async fn auto_moderation_rule_delete(&self, ctx: Context, rule: Rule) {run!("auto_moderation_rule_delete", ctx, rule);}
    async fn auto_moderation_action_execution(&self, ctx: Context, execution: ActionExecution) {run!("auto_moderation_action_execution", ctx, execution);}
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {run!("cache_ready", ctx, guilds);}
    async fn shards_ready(&self, ctx: Context, total_shards: u32) {run!("shards_ready", ctx, total_shards);}
    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {run!("channel_create", ctx, channel);}
    async fn category_create(&self, ctx: Context, category: GuildChannel) {run!("category_create", ctx, category);}
    async fn category_delete(&self, ctx: Context, category: GuildChannel) {run!("category_delete", ctx, category);}
    async fn channel_delete(&self, ctx: Context, channel: GuildChannel, messages: Option<Vec<Message>>) {run!("channel_create", ctx, channel, messages);}
    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {run!("channel_pins_update", ctx, pin);}
    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {run!("channel_update", ctx, old, new);}
    async fn guild_audit_log_entry_create(&self, ctx: Context, entry: AuditLogEntry, guild_id: GuildId) {run!("guild_audit_log_entry_create", ctx, entry, guild_id);}
    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {run!("guild_ban_addition", ctx, guild_id, banned_user);}
    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {run!("guild_ban_removal", ctx, guild_id, unbanned_user);}
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {run!("guild_create", ctx, guild, is_new);}
    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {run!("guild_delete", ctx, incomplete, full);}
    async fn guild_emojis_update(&self, ctx: Context, guild_id: GuildId, current_state: HashMap<EmojiId, Emoji>) {run!("guild_emojis_update", ctx, guild_id, current_state);}
    async fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {run!("guild_integrations_update", ctx, guild_id);}
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {run!("guild_member_addition", ctx, new_member);}
    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, member_data_if_available: Option<Member>) {run!("guild_member_removal", ctx, guild_id, user, member_data_if_available);}
    async fn guild_member_update(&self, ctx: Context, old_if_available: Option<Member>, new: Option<Member>, event: GuildMemberUpdateEvent) {run!("guild_member_update", ctx, old_if_available, new, event);}
    async fn guild_members_chunk(&self, ctx: Context, chunk: GuildMembersChunkEvent) {run!("guild_members_chunk", ctx, chunk);}
    async fn guild_role_create(&self, ctx: Context, new: Role) {run!("guild_role_create", ctx, new);}
    async fn guild_role_delete(&self, ctx: Context, guild_id: GuildId, removed_role_id: RoleId, removed_role_date_if_available: Option<Role>) {run!("guild_role_delete", ctx, guild_id, removed_role_id, removed_role_date_if_available);}
    async fn guild_role_update(&self, ctx: Context, old_data_if_available: Option<Role>, new: Role) {run!("guild_role_update", ctx, old_data_if_available, new);}
    async fn guild_stickers_update(&self, ctx: Context, guild_id: GuildId, current_state: HashMap<StickerId, Sticker>) {run!("guild_stickers_update", ctx, guild_id, current_state);}
    async fn guild_update(&self, ctx: Context, old_data_if_available: Option<Guild>, new_data: PartialGuild) {run!("guild_update", ctx, old_data_if_available, new_data);}
    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {run!("invite_create", ctx, data);}
    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {run!("invite_delete", ctx, data);}
    async fn message(&self, ctx: Context, new_message: Message) {run!("message", ctx, new_message);}
    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {run!("message_delete", ctx, channel_id, deleted_message_id, guild_id);}
    async fn message_delete_bulk(&self, ctx: Context, channel_id: ChannelId, multiple_deleted_messages_ids: Vec<MessageId>, guild_id: Option<GuildId>) {run!("message_delete_bulk", ctx, channel_id, multiple_deleted_messages_ids, guild_id);}
    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {run!("message_update", ctx, old_if_available, new, event);}
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {run!("reaction_add", ctx, add_reaction);}
    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {run!("reaction_remove", ctx, removed_reaction);}
    async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, removed_from_message_id: MessageId) {run!("reaction_remove_all", ctx, channel_id, removed_from_message_id);}
    async fn reaction_remove_emoji(&self, ctx: Context, removed_reactions: Reaction) {run!("reaction_remove_emoji", ctx, removed_reactions);}
    async fn presence_update(&self, ctx: Context, new_data: Presence) {run!("presence_update", ctx, new_data);}
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {run!("ready", ctx, data_about_bot);}
    async fn resume(&self, ctx: Context, event: ResumedEvent) {run!("resume", ctx, event);}
    async fn shard_stage_update(&self, ctx: Context, event: ShardStageUpdateEvent) {run!("shard_state_update", ctx, event);}
    async fn soundboard_sounds(&self, ctx: Context, event: SoundboardSoundsEvent) {run!("soundboard_sounds", ctx, event);}
    async fn soundboard_sound_create(&self, ctx: Context, event: SoundboardSoundCreateEvent) {run!("soundboard_sound_create", ctx, event);}
    async fn soundboard_sound_update(&self, ctx: Context, event: SoundboardSoundUpdateEvent) {run!("soundboard_sound_update", ctx, event);}
    async fn soundboard_sounds_update(&self, ctx: Context, event: SoundboardSoundsUpdateEvent) {run!("soundboard_sounds_update", ctx, event);}
    async fn soundboard_sound_delete(&self, ctx: Context, event: SoundboardSoundDeleteEvent) {run!("soundboard_sound_delete", ctx, event);}
    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {run!("typing_start", ctx, event);}
    async fn user_update(&self, ctx: Context, old_data: Option<CurrentUser>, new: CurrentUser) {run!("user_update", ctx, old_data, new);}
    async fn voice_server_update(&self, ctx: Context, event: VoiceServerUpdateEvent) {run!("voice_server_update", ctx, event);}
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {run!("voice_state_update", ctx, old, new);}
    async fn voice_channel_status_update(&self, ctx: Context, old: Option<String>, status: Option<String>, id: ChannelId, guild_id: GuildId) {run!("voice_channel_status_update", ctx, old, status, id, guild_id);}
    async fn webhook_update(&self, ctx: Context, guild_id: GuildId, belongs_to_channnel_id: ChannelId) {run!("webhook_update", ctx, guild_id, belongs_to_channnel_id);}
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {run!("interaction_create", ctx, interaction);}
    async fn integration_create(&self, ctx: Context, integration: Integration) {run!("integration_create", ctx, integration);}
    async fn integration_update(&self, ctx: Context, integration: Integration) {run!("integration_update", ctx, integration);}
    async fn integration_delete(&self, ctx: Context, integration_id: IntegrationId, guild_id: GuildId, application_id: Option<ApplicationId>) {run!("integration_delete", ctx, integration_id, guild_id, application_id);}
    async fn stage_instance_create(&self, ctx: Context, stage_instance: StageInstance) {run!("stage_instance_create", ctx, stage_instance);}
    async fn stage_instance_update(&self, ctx: Context, stage_instance: StageInstance) {run!("stage_instance_update", ctx, stage_instance);}
    async fn stage_instance_delete(&self, ctx: Context, stage_instance: StageInstance) {run!("stage_instance_delete", ctx, stage_instance);}
    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {run!("thread_create", ctx, thread);}
    async fn thread_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {run!("thread_update", ctx, old, new);}
    async fn thread_delete(&self, ctx: Context, thread: PartialGuildChannel, full_thread_data: Option<GuildChannel>) {run!("thread_delete", ctx, thread, full_thread_data);}
    async fn thread_list_sync(&self, ctx: Context, thread_list_sync: ThreadListSyncEvent) {run!("thread_list_sync", ctx, thread_list_sync);}
    async fn thread_member_update(&self, ctx: Context, thread_member: ThreadMember) {run!("thread_member_update", ctx, thread_member);}
    async fn thread_members_update(&self, ctx: Context, thread_members_update: ThreadMembersUpdateEvent) {run!("thread_members_update", ctx, thread_members_update);}
    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {run!("guild_scheduled_event_create", ctx, event);}
    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {run!("guild_scheduled_event_update", ctx, event);}
    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {run!("guild_scheduled_event_delete", ctx, event);}
    async fn guild_scheduled_event_user_add(&self, ctx: Context, subscribed: GuildScheduledEventUserAddEvent) {run!("guild_scheduled_event_user_add", ctx, subscribed);}
    async fn guild_scheduled_event_user_remove(&self, ctx: Context, unsubscribed: GuildScheduledEventUserRemoveEvent) {run!("guild_scheduled_event_user_remove", ctx, unsubscribed);}
    async fn entitlement_create(&self, ctx: Context, entitlement: Entitlement) {run!("entitlement_create", ctx, entitlement);}
    async fn entitlement_update(&self, ctx: Context, entitlement: Entitlement) {run!("entitlement_update", ctx, entitlement);}
    async fn entitlement_delete(&self, ctx: Context, entitlement: Entitlement) {run!("entitlement_delete", ctx, entitlement);}
    async fn poll_vote_add(&self, ctx: Context, event: MessagePollVoteAddEvent) {run!("poll_vote_add", ctx, event);}
    async fn poll_vote_remove(&self, ctx: Context, event: MessagePollVoteRemoveEvent) {run!("poll_vote_remove", ctx, event);}
}

