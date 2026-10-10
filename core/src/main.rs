extern crate serenity;
extern crate reg;

use std::sync::LazyLock;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use reg::*;
use tokio::sync::Notify;

static KEY: LazyLock<std::sync::Mutex<String>> = LazyLock::new( || {std::sync::Mutex::new(String::new())});
static NOTIF: LazyLock<Notify> = LazyLock::new(|| {Notify::new()});

#[tokio::main]
async fn main() {
    let mut reader: BufReader<io::Stdin> = BufReader::new(io::stdin());

    let mut client: Client =
        Client::builder(TOKEN, GatewayIntents::all())
        .event_handler(Handler).await.unwrap();
    
    let mut key: String = String::new();
    let _ = reader.read_line(&mut key).await;
    let bot_info: Vec<&str> = key.split(' ').collect();
    if let Ok(mut v) = KEY.lock() {
        v.push_str(bot_info[0]);
    }

    // populate client

    tokio::spawn(async move {client.start().await.unwrap()});
    NOTIF.notified().await;
    let _ = reader.read_line(&mut String::new()).await;
    reg::INSTANCE_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
    NOTIF.notify_one();
    let _ = reader.read_line(&mut String::new()).await;
}

#[event(reconn)]
async fn reconn(ctx: Context, event: ResumedEvent) {
    if !reg::INSTANCE_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
        if let Ok(v) = KEY.lock() {
            println!("{v}");
        }
        NOTIF.notify_one();
        NOTIF.notified().await;
    }
    run_event!("resume", ctx, event);
}

static TOKEN: &str = "";

#[acmd]
async fn g(ctx: Context, interaction: CommandInteraction, num: Option<User>) {
    if let Some(user) = num {
        let _ = interaction.response(&ctx, &format!("hi {}", user.name), false).await;
    } else {
        let _ = interaction.response(&ctx, &format!("hi"), false).await;
    }
}

/*
use serenity::prelude::*;
use serenity::gateway::SessionInfo;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    
    // 1. Fetch the handed-over state from your Parent process via IPC/Args
    let handed_over_json = fetch_state_from_parent().await; 
    
    // 2. Extract Gateway session data
    let session_id = handed_over_json["session_id"].as_str().unwrap().to_string();
    let sequence_number = handed_over_json["sequence_number"].as_u64().unwrap();

    // 3. Build Serenity client
    let mut client = Client::builder(&token, GatewayIntents::non_privileged())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // 4. INJECT THE COLD START RESUME DATA
    {
        let shard_manager = client.shard_manager.clone();
        let mut runners = shard_manager.runners.lock().await;
        
        if let Some(runner) = runners.get_mut(&ShardId(0)) {
            // By populating this, Serenity skips the IDENTIFY phase 
            // and goes straight to sending a RESUME packet on startup
            runner.session_id = Some(session_id);
            runner.sequence_number = Some(sequence_number);
        }
    }

    // 5. Start the bot safely
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
*/

/*
use serenity::gateway::ShardId;

// Inside your poise/serenity command or IPC handler
pub async fn prepare_handoff(ctx: &serenity::prelude::Context) -> String {
    let shard_manager = ctx.shard_manager.clone();
    let runners = shard_manager.runners.lock().await;
    
    // Assuming a single-sharded bot (Shard 0)
    if let Some(runner) = runners.get(&ShardId(0)) {
        let session_id = runner.session_id.clone();
        let sequence_number = runner.sequence_number;
        
        // Bundle this into a JSON string along with your bot's internal database cache
        let state_json = serde_json::json!({
            "session_id": session_id,
            "sequence_number": sequence_number,
            "custom_bot_state": { "active_trivia_games": [] }
        });
        
        return state_json.to_string();
    }
    
    unreachable!("Shard 0 should exist")
}

*/