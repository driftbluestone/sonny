use serde::{Deserialize, Serialize};

const DISCORD_EPOCH: u64 = 1420070400000;

/* Snowflake */
/// Reference:  https://docs.discord.com/developers/reference#snowflakes
/// 
/// Implements to/from String and u64.
/// Can also be converted to timestamps.
#[derive(Serialize, Deserialize)]
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

#[allow(unused)]
pub mod tests {
    pub fn snowflake() {
        use crate::structs::base::Snowflake;
        use serde_json::to_string;

        let s: Snowflake = Snowflake::from(1234123412341);
        let st = to_string(&s);
        println!("{:?}", st);
        let st = st.unwrap();
        println!("{:?}", st);
        let st: String = s.into();
        println!("{}", st);
    }
}