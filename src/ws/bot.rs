use std::{error::Error, fmt::Write, sync::{Arc, atomic::{AtomicU64, Ordering::Relaxed}}};
use futures_util::{SinkExt, StreamExt};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use tokio::{net::TcpStream, sync::RwLock, task::JoinHandle, time::{self, Duration}};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message::{self, Text}};

type WebSocketWriter = futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>;

#[derive(serde::Deserialize, serde::Serialize)]
struct GatewayResponse {
    url: String,
    session_start_limit: SSL,
    shards: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct SSL {
    max_concurrency: i64,
    remaining: i64,
    reset_after: i64,
    total: i64
}

#[derive(serde::Serialize)]
struct IdentifyData {
    token: String,
    intents: u64,
    properties: IdentifyProperties,
}

#[derive(serde::Serialize)]
struct IdentifyProperties {
    os: String,
    browser: String,
    device: String,
}

#[derive(serde::Serialize)]
struct GatewayPayload<T> {
    op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    d: Option<T>,
}

#[derive(Clone)]
pub struct Bot {
    s: Arc<AtomicU64>,
    ws: Arc<RwLock<Option<WebSocketWriter>>>,
    intents: u64,
    token: String
}
macro_rules! api {
    ($e:expr) => {
        concat!("https://discord.com/api/v10", $e)
    };
}

impl Bot {
    pub fn new(token: &str, intents: u64) -> Self {
        Self {
            s: Arc::new(AtomicU64::new(0)),
            token: token.to_string(),
            intents: intents,
            ws: Arc::new(RwLock::new(None)),
        }
    }

    #[allow(unused)]
    pub async fn start(&self) -> JoinHandle<()> {
        let mut bot = self.clone();
        tokio::spawn(async move {
            let _ = bot.start_blocking().await;
        })
    }

    pub async fn start_blocking(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let client: reqwest::Client = reqwest::Client::new();
        let token = &self.token.clone();

        let response:reqwest::Response = client
        .get(api!("/gateway/bot"))
        .header(AUTHORIZATION, format!("Bot {}", token))
        .header(USER_AGENT, "DiscordBot (https://github.com/driftbluestone/sonny, 1.0.0)")
        .send().await?;
        
        println!("Status: {}", response.status());
        let body: String = response.text().await?.trim().to_string();
        println!("Response:\n{}", body);
        let url: GatewayResponse = serde_json::from_str(&body).unwrap();
        let url = format!("{}/?v=10&encoding=json", url.url);
        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await?;
        let (write, mut read) = ws_stream.split();
        self.ws.write().await.replace(write);
        
        while let Some(msg) = read.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(text) = msg? {
                let json: serde_json::Value = serde_json::from_str(&text)?;
                let op = json["op"].as_u64().unwrap_or(99);

                // Opcode 10: Hello
                if op == 10 {
                    let interval = json["d"]["heartbeat_interval"].as_u64().unwrap_or(41351);
                    println!("Received Hello. Heartbeat interval: {}ms", interval);

                    // Send Identify (Opcode 2)
                    let identify: GatewayPayload<IdentifyData> = GatewayPayload {
                        op: 2,
                        d: Some(IdentifyData {
                            token: token.clone(),
                            intents: self.intents,
                            properties: IdentifyProperties {
                                os: "linux".to_string(),
                                browser: "temp".to_string(),
                                device: "temp".to_string(),
                            },
                        }),
                    };

                    let payload_str = serde_json::to_string(&identify)?;
                    self.send(tokio_tungstenite::tungstenite::Message::Text(payload_str.into())).await;
                    println!("did the identify");

                    let bot = self.clone();
                    tokio::spawn(async move {
                        bot.hb_loop(interval).await;
                    });

                    // Note: In a production bot, spawn a background task here to send
                    // Opcode 1 (Heartbeat) every `interval` milliseconds with the last sequence number.
                }

                // Opcode 0: Dispatch Event
                if op == 0 {
                    let event_name = json["t"].as_str().unwrap_or("UNKNOWN");
                    self.update_s(json["s"].as_u64().unwrap_or(0)).await;
                    println!("Dispatch event: {}", event_name);

                    if event_name == "MESSAGE_CREATE" {
                        let content = json["d"]["content"].as_str().unwrap_or("");
                        let author = json["d"]["author"]["username"].as_str().unwrap_or("");
                        println!("Message from {}: {}", author, content);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn get_s(&self) -> Option<u64> {
        let s = self.s.load(Relaxed);
        if s == 0 {
            return None;   
        }
        Some(s)
    }

    async fn update_s(&self, s: u64) {
        self.s.store(s, Relaxed);
    }

    // heartbeat: opcode 1
    async fn hb_loop(&self, interval_ms: u64) {
        let mut interval: time::Interval = time::interval(Duration::from_millis(interval_ms));
        
        loop {
            self.send_hb().await;
            interval.tick().await;
        }
    }

    async fn send_hb(&self) {
        let payload = match self.get_s().await {
            Some(seq) => {
                let mut buf: String = String::with_capacity(34);
                let _ = write!(buf, "{{\"op\":1,\"d\":{seq}}}");
                buf.into()
            }
            None => "{\"op\":1,\"d\":null}".into(),
        };

        let _ = self.send(Text(payload)).await;
    }

    async fn send(&self, msg: Message) {
        let _ = self.ws.write().await.as_mut().unwrap().send(msg).await;
    }
}
