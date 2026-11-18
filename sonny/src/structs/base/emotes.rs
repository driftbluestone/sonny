use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::structs::base::snowflake::Snowflake;
use crate::structs::base::base::UserBase;

type Nullable<T> = Option<T>;
type NullableOption<T> = Option<T>;

/// Reference: https://docs.discord.com/developers/resources/emoji#emoji-object
#[derive(Serialize, Deserialize)]
pub struct EmojiBase {
    id: Nullable<Snowflake>,
    name: Nullable<String>,
    roles: Option<Vec<Snowflake>>, // roles allowed to use thsi emoji
    user: Option<UserBase>, // user that created this emoji
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}

/// Reference: https://docs.discord.com/developers/resources/sticker#sticker-object
#[derive(Serialize, Deserialize)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Option<Snowflake>,
    name: String,
    description: Nullable<String>,
    tags: String, // csv, apparently
    r#type: StickerType,
    format_type: StickerFormatType,
    available: Option<bool>,
    guild_id: Option<Snowflake>,
    user: Option<UserBase>,
    sort_value: Option<i64>,
}

/// Reference: https://docs.discord.com/developers/resources/sticker#sticker-item-object
#[derive(Serialize, Deserialize)]
pub struct StickerItem {
    id: Snowflake,
    name: String,
    format_type: StickerFormatType,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum StickerType {
    Standard = 1,
    Guild,
}

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum StickerFormatType {
    PNG = 1,
    APNG,
    LOTTIE,
    GIF,
}