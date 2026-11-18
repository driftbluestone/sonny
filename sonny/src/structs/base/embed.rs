use crate::structs::base::snowflake::*;

/// Reference: https://docs.discord.com/developers/resources/message#embed-object
#[derive(Serialize, Deserialize)]
pub struct Embed {
    title: Option<String>,
    r#type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    #[serde(with = "time::serde::iso8601::option")]
    timestamp: Option<OffsetDateTime>,
    color: Option<u32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
    components: Option<Vec<()>>, // TBI: https://docs.discord.com/developers/components/reference
    flags: Option<u32>,
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-footer-structure
#[derive(Serialize, Deserialize)]
struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-image-structure
#[derive(Serialize, Deserialize)]
struct EmbedImage {
    url: String,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    content_type: Option<String>,
    placeholder: Option<String>,
    placeholder_version: Option<u32>,
    description: Option<String>,
    flags: Option<u32>, // bitfield: https://docs.discord.com/developers/resources/message#embed-object-embed-media-flags
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-video-structure
#[derive(Serialize, Deserialize)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    content_type: Option<String>,
    placeholder: Option<String>,
    placeholder_version: Option<u32>,
    description: Option<String>,
    flags: Option<u32>, // bitfield: https://docs.discord.com/developers/resources/message#embed-object-embed-media-flags
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-provider-structure
#[derive(Serialize, Deserialize)]
struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-author-structure
#[derive(Serialize, Deserialize)]
struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

/// Reference: https://docs.discord.com/developers/resources/message#embed-object-embed-field-structure
#[derive(Serialize, Deserialize)]
struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}
