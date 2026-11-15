#[derive(serde::Deserialize, serde::Serialize)]
pub struct GatewayResponse {
    pub url: String,
    pub session_start_limit: SSL,
    pub shards: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SSL {
    pub max_concurrency: i64,
    pub remaining: i64,
    pub reset_after: i64,
    pub total: i64
}

#[derive(serde::Serialize)]
pub struct IdentifyData {
    pub token: String,
    pub intents: u64,
    pub properties: IdentifyProperties,
}

#[derive(serde::Serialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(serde::Serialize)]
pub struct GatewayPayload<T> {
    pub op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<T>,
}