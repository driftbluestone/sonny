use crate::structs::base::{application::Application, base::UserBase, snowflake::*};

/// Reference: https://docs.discord.com/developers/resources/message#attachment-object
#[derive(Serialize, Deserialize)]
pub struct Attachment {
    id: Snowflake,
    filename: String,
    title: Option<String>,
    description: Option<String>,
    content_type: Option<String>,
    size: u64,
    url: String,
    proxy_url: String,
    height: NullableOption<u32>,
    width: NullableOption<u32>,
    placeholder: Option<String>,
    placeholder_version: Option<u32>,
    ephemeral: Option<bool>,
    duration_secs: Option<f64>,
    waveform: Option<String>,
    flags: Option<u32>, // bitfield: https://docs.discord.com/developers/resources/message#attachment-object-attachment-flags
    clip_participants: Option<Vec<UserBase>>,
    #[serde(with = "time::serde::iso8601::option")]
    clip_created_at: Option<OffsetDateTime>,
    application: NullableOption<Application>,
}
