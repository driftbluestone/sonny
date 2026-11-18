use crate::structs::base::{emotes::EmojiBase, snowflake::*};

/// Reference: https://docs.discord.com/developers/components/reference#component-object
#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[repr(u8)]
pub enum ComponentType {
    ActionRow(ActionRow) = 1,
    Button(Button) = 2,
    StringSelect(StringSelect) = 3,

}

/// Reference: https://docs.discord.com/developers/components/reference#action-row
#[derive(Deserialize, Serialize)]
pub struct ActionRow {
    id: Option<i64>,
    components: Vec<ComponentType>,
}

/// Reference: https://docs.discord.com/developers/components/reference#button
/// 
/// Some fields only apply to some button types. See reference.
#[derive(Deserialize, Serialize)]
pub struct Button {
    id: Option<i64>,
    style: ButtonStyle,
    label: Option<String>,
    emoji: Option<EmojiBase>, // partial
    custom_id: Option<String>,
    sku_id: Option<Snowflake>,
    url: Option<String>,
    disabled: Option<bool>, // defaults to false
}

/// Reference: https://docs.discord.com/developers/components/reference#button-button-styles
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary,
    Success,
    Danger,
    Link,
    Premium,
}

/// Reference: https://docs.discord.com/developers/components/reference#string-select
#[derive(Deserialize, Serialize)]
pub struct StringSelect {
    id: Option<i64>,
    custom_id: String,
    options: Vec<SelectOption>,
    placeholder: Option<String>,
    min_values: Option<u8>, // 1 ~ 25, must be omitted or at least 1 if `required` is omitted or `true`
    max_values: Option<u8>, // 1 ~ 25, defaults to 1
    required: Option<bool>, // only for StringSelect in Modals, will be ignored in messages
}

/// Reference: https://docs.discord.com/developers/components/reference#string-select-select-option-structure
#[derive(Deserialize, Serialize)]
pub struct SelectOption {
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<EmojiBase>, // partial
    default: Option<bool>,
}