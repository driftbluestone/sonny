#![allow(unused_imports)]
pub use serde::{Deserialize, Serialize};
pub use serde_repr::{Deserialize_repr, Serialize_repr};

pub use time::OffsetDateTime;

pub const DISCORD_EPOCH: u64 = 1420070400000;

pub type Nullable<T> = Option<T>;
pub type NullableOption<T> = Option<T>;

/// Reference:  https://docs.discord.com/developers/reference#snowflakes
/// 
/// Implements to/from String and u64.
/// Can also be converted to timestamps.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Snowflake(String);

impl TryFrom<String> for Snowflake {
    type Error = std::num::ParseIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<u64>()?;
        Ok(Snowflake ( s ))
    }
}

impl From<u64> for Snowflake {
    fn from(i: u64) -> Self {
        Snowflake(i.to_string())
    }
}

impl From<Snowflake> for String {
    fn from(s: Snowflake) -> Self {
        s.0
    }
}

impl From<Snowflake> for u64 {
    fn from(s: Snowflake) -> Self {
        s.0.parse().unwrap()
    }
}

#[allow(unused)]
impl Snowflake {
    fn to_timestamp_ms(self) -> u64 {
        let n: u64 = self.into();
        (n >> 22) + DISCORD_EPOCH
    }

    fn to_timestamp_s(self) -> u64 {
        self.to_timestamp_ms() / 1000
    }
}