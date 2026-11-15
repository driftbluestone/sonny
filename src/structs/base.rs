use serde::{Deserialize, Serialize};

const DISCORD_EPOCH: u64 = 1420070400000;
/* Snowflake */
/// Reference:  https://docs.discord.com/developers/reference#snowflakes
#[derive(Serialize, Deserialize)]
struct Snowflake {
    value: String
}

impl From<u64> for Snowflake {
    fn from(i: u64) -> Self {
        Snowflake {value:i.to_string()}
    }
}

impl From<String> for Snowflake {
    fn from(s: String) -> Self {
        Snowflake { value: s }
    }
}

impl From<Snowflake> for String {
    fn from(s: Snowflake) -> Self {
        s.value
    }
}

impl From<Snowflake> for u64 {
    fn from(s: Snowflake) -> Self {
        s.value.parse().unwrap()
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