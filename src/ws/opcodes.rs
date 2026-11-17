use tracing::info;

use crate::ws::{structs::*, bot::*};

type Json = serde_json::Value;

pub trait Opcode {
    async fn opcode(&self, op: u64, json: Json);
    // opcode reference:
    // https://docs.discord.com/developers/topics/opcodes-and-status-codes#gateway

    // gateway opcodes
    async fn opcode_0 (&self, content: Json);
    async fn opcode_1 (&self, content: Json);
    async fn opcode_7 (&self, content: Json);
    async fn opcode_9 (&self, content: Json);
    async fn opcode_10(&self, content: Json);
    async fn opcode_11(&self, content: Json);

    // gateway event close codes
    async fn opcode_4000(&self, content: Json);
    async fn opcode_4001(&self, content: Json);
    async fn opcode_4002(&self, content: Json);
    async fn opcode_4003(&self, content: Json);
    async fn opcode_4004(&self, content: Json);
    async fn opcode_4005(&self, content: Json);
    async fn opcode_4006(&self, content: Json);
    async fn opcode_4007(&self, content: Json);
    async fn opcode_4008(&self, content: Json);
    async fn opcode_4009(&self, content: Json);
    async fn opcode_4010(&self, content: Json);
    async fn opcode_4011(&self, content: Json);
    async fn opcode_4012(&self, content: Json);
    async fn opcode_4013(&self, content: Json);
    async fn opcode_4014(&self, content: Json);

}

impl Opcode for Bot {
    async fn opcode(&self, op: u64, content: Json) {
        match op {
            // Some opcodes are meant to be sent TO
            // Discord, so they arent included.
            0  => self.opcode_0 (content).await,
            1  => self.opcode_1 (content).await,
            7  => self.opcode_7 (content).await,
            9  => self.opcode_9 (content).await,
            10 => self.opcode_10(content).await,
            11 => self.opcode_11(content).await,

            4000 => self.opcode_4000(content).await,
            4001 => self.opcode_4001(content).await,
            4002 => self.opcode_4002(content).await,
            4003 => self.opcode_4003(content).await,
            4004 => self.opcode_4004(content).await,
            4005 => self.opcode_4005(content).await,
            4006 => self.opcode_4006(content).await,
            4007 => self.opcode_4007(content).await,
            4008 => self.opcode_4008(content).await,
            4009 => self.opcode_4009(content).await,
            4010 => self.opcode_4010(content).await,
            4011 => self.opcode_4011(content).await,
            4012 => self.opcode_4012(content).await,
            4013 => self.opcode_4013(content).await,
            4014 => self.opcode_4014(content).await,
            _ => info!("Recieved unimplemented opcode: {}", op)
        }
    }

    // actual discord event
    async fn opcode_0(&self, content: Json) { self.dispatch(content).await; }

    // heartbeat on request
    async fn opcode_1(&self, _content: Json) { self.send_hb().await; }

    // these mean that we need to try reconnecting
    async fn opcode_7(&self, _content: Json) {}
    async fn opcode_9(&self, _content: Json) {}

    async fn opcode_10(&self, content: Json) {
        let interval = content["d"]["heartbeat_interval"].as_u64().unwrap_or(41351);
            info!("Received Hello. Heartbeat interval: {}ms", interval);

            // Send Identify (Opcode 2)
            let identify: GatewayPayload<IdentifyData> = GatewayPayload {
                op: 2,
                d: Some(IdentifyData {
                    token: self.token.clone(),
                    intents: self.intents,
                    properties: IdentifyProperties {
                        os: "linux".to_string(),
                        browser: "temp".to_string(),
                        device: "temp".to_string(),
                    },
                }),
            };

            let payload_str = serde_json::to_string(&identify).unwrap();
            self.send(tokio_tungstenite::tungstenite::Message::Text(payload_str.into())).await;
            info!("did the identify");

            let bot = self.clone();
            tokio::spawn(async move {
                bot.hb_loop(interval).await;
            });
    }

    // heartbeat ACK
    // should add some impl for when we go
    // ~3 heartbeats without any op11's
    async fn opcode_11(&self, _content: Json) {}

    async fn opcode_4000(&self, _content: Json) {}
    async fn opcode_4001(&self, _content: Json) {}
    async fn opcode_4002(&self, _content: Json) {}
    async fn opcode_4003(&self, _content: Json) {}
    async fn opcode_4004(&self, _content: Json) {}
    async fn opcode_4005(&self, _content: Json) {}
    async fn opcode_4006(&self, _content: Json) {}
    async fn opcode_4007(&self, _content: Json) {}
    async fn opcode_4008(&self, _content: Json) {}
    async fn opcode_4009(&self, _content: Json) {}
    async fn opcode_4010(&self, _content: Json) {}
    async fn opcode_4011(&self, _content: Json) {}
    async fn opcode_4012(&self, _content: Json) {}
    async fn opcode_4013(&self, _content: Json) {}
    async fn opcode_4014(&self, _content: Json) {}
}