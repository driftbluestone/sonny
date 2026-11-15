use crate::ws::{structs::*, bot::*};
pub trait Opcode {
    async fn op(&self, op: u64, json: serde_json::Value);
    // https://docs.discord.com/developers/topics/opcodes-and-status-codes#gateway

    // gateway opcodes
    async fn op0(&self, json: serde_json::Value);
    async fn op1(&self, json: serde_json::Value);
    async fn op7(&self, json: serde_json::Value);
    async fn op9(&self, json: serde_json::Value);
    async fn op10(&self, json: serde_json::Value);
    async fn op11(&self, json: serde_json::Value);

    // gateway event close codes
    async fn op4000(&self, json: serde_json::Value);
    async fn op4001(&self, json: serde_json::Value);
    async fn op4002(&self, json: serde_json::Value);
    async fn op4003(&self, json: serde_json::Value);
    async fn op4004(&self, json: serde_json::Value);
    async fn op4005(&self, json: serde_json::Value);
    async fn op4006(&self, json: serde_json::Value);
    async fn op4007(&self, json: serde_json::Value);
    async fn op4008(&self, json: serde_json::Value);
    async fn op4009(&self, json: serde_json::Value);
    async fn op4010(&self, json: serde_json::Value);
    async fn op4011(&self, json: serde_json::Value);
    async fn op4012(&self, json: serde_json::Value);
    async fn op4013(&self, json: serde_json::Value);
    async fn op4014(&self, json: serde_json::Value);

}

impl Opcode for Bot {
    async fn op(&self, op: u64, json: serde_json::Value) {
        match op {
            // Some opcodes are meant to be sent TO
            // Discord, so they arent included.
            0  => self.op0 (json).await,
            1  => self.op1 (json).await,
            7  => self.op7 (json).await,
            9  => self.op9 (json).await,
            10 => self.op10(json).await,
            11 => self.op11(json).await,

            4000 => self.op4000(json).await,
            4001 => self.op4001(json).await,
            4002 => self.op4002(json).await,
            4003 => self.op4003(json).await,
            4004 => self.op4004(json).await,
            4005 => self.op4005(json).await,
            4006 => self.op4006(json).await,
            4007 => self.op4007(json).await,
            4008 => self.op4008(json).await,
            4009 => self.op4009(json).await,
            4010 => self.op4010(json).await,
            4011 => self.op4011(json).await,
            4012 => self.op4012(json).await,
            4013 => self.op4013(json).await,
            4014 => self.op4014(json).await,
            _ => println!("Recieved unimplemented opcode: {}", op)
        }
    }

    // actual discord event
    async fn op0(&self, json: serde_json::Value) { self.dispatch(json).await; }

    // heartbeat on request
    async fn op1(&self, _json: serde_json::Value) { self.send_hb().await; }

    // these mean that we need to try reconnecting
    async fn op7(&self, _json: serde_json::Value) {}
    async fn op9(&self, _json: serde_json::Value) {}

    async fn op10(&self, json: serde_json::Value) {
        let interval = json["d"]["heartbeat_interval"].as_u64().unwrap_or(41351);
            println!("Received Hello. Heartbeat interval: {}ms", interval);

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
            println!("did the identify");

            let bot = self.clone();
            tokio::spawn(async move {
                bot.hb_loop(interval).await;
            });
    }

    // heartbeat ACK
    // should add some impl for when we go
    // ~3 heartbeats without any op11's
    async fn op11(&self, _json: serde_json::Value) {}

    async fn op4000(&self, _json: serde_json::Value) {}
    async fn op4001(&self, _json: serde_json::Value) {}
    async fn op4002(&self, _json: serde_json::Value) {}
    async fn op4003(&self, _json: serde_json::Value) {}
    async fn op4004(&self, _json: serde_json::Value) {}
    async fn op4005(&self, _json: serde_json::Value) {}
    async fn op4006(&self, _json: serde_json::Value) {}
    async fn op4007(&self, _json: serde_json::Value) {}
    async fn op4008(&self, _json: serde_json::Value) {}
    async fn op4009(&self, _json: serde_json::Value) {}
    async fn op4010(&self, _json: serde_json::Value) {}
    async fn op4011(&self, _json: serde_json::Value) {}
    async fn op4012(&self, _json: serde_json::Value) {}
    async fn op4013(&self, _json: serde_json::Value) {}
    async fn op4014(&self, _json: serde_json::Value) {}
}