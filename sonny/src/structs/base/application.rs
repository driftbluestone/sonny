use std::collections::HashMap;

use crate::structs::base::{
    base::{GuildBase, UserBase},
    snowflake::*,
};

/// Reference: https://docs.discord.com/developers/resources/application#application-object
#[derive(Serialize, Deserialize)]
pub struct Application {
    id: Snowflake,
    name: String,
    icon: Nullable<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    bot: Option<UserBase>, // (partial)
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<UserBase>, // (partial)
    verify_key: String,
    team: Nullable<Team>,
    guild_id: Option<Snowflake>,
    guild: Option<GuildBase>, // (partial)d
    primary_sku_id: Option<Snowflake>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<u32>, // legacy, bitfield: https://docs.discord.com/developers/resources/application#application-object-application-flags
    flags_new: Option<String>, // bitfield: https://docs.discord.com/developers/resources/application#application-object-application-flags
    approximate_guild_count: Option<u32>,
    approximate_user_install_count: Option<u32>,
    approximate_user_authorization_count: Option<u32>,
    redirect_uris: Option<Vec<String>>,
    interactions_endpoint_url: NullableOption<String>,
    role_connections_verification_url: NullableOption<String>,
    event_webhooks_url: NullableOption<String>,
    event_webhooks_status: Option<EventWebhookStatus>,
    event_webhooks_types: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    install_params: Option<InstallParams>,
    integration_types_config: Option<HashMap<IntegrationType, IntegrationTypeConfiguration>>,
    custom_install_url: Option<String>,
}

/// Reference: https://docs.discord.com/developers/topics/teams#data-models-team-object
#[derive(Serialize, Deserialize)]
struct Team {
    icon: Nullable<String>,
    id: Snowflake,
    members: Vec<TeamMember>,
    name: String,
    owner_user_id: Snowflake,
}

/// Reference: https://docs.discord.com/developers/topics/teams#data-models-team-member-object
#[derive(Serialize, Deserialize)]
struct TeamMember {
    membership_state: MembershipState,
    team_id: Snowflake,
    user: UserBase,
    role: String,
}

/// Reference: https://docs.discord.com/developers/topics/teams#data-models-membership-state-enum
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum MembershipState {
    Invited = 1,
    Accepted = 2,
}

/// Reference: https://docs.discord.com/developers/resources/application#application-object-application-event-webhook-status
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum EventWebhookStatus {
    Disabled = 1,
    Enabled = 2,
    DisabledByDiscord = 3,
}

/// Reference: https://docs.discord.com/developers/resources/application#install-params-object
#[derive(Serialize, Deserialize)]
struct InstallParams {
    scopes: Vec<String>,
    permissions: String,
}

/// Reference: https://docs.discord.com/developers/resources/application#application-object-application-integration-types
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IntegrationType {
    GuildInstall = 0,
    UserInstall = 1,
}

/// Reference: https://docs.discord.com/developers/resources/application#install-params-object
#[derive(Serialize, Deserialize)]
struct IntegrationTypeConfiguration {
    oauth2_install_params: Option<InstallParams>,
}
