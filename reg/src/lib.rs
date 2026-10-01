pub use pma::*;
pub use registry::*;

#[doc(hidden)]
pub mod __private {
    pub use ctor;
    pub use registry;
}

extern crate self as reg;

use serenity::model::prelude::*;
use serenity::prelude::*;

macro_rules! run_cmd {
    ($cmd:expr, $($args:expr),*) => {{
        if let Ok(registry) = CMD_REG.lock() {
            if let Some(callback) = registry.get($cmd) {
                let mut callback_args: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
                $(
                    callback_args.push(Box::new($args.clone()) as Box<dyn std::any::Any + Send + Sync>);
                )*

                tokio::spawn(callback(callback_args));
            }
        }
    }};
}

macro_rules! cmd_exists {
    ($cmd:expr) => {(||{
        if let Ok(registry) = CMD_REG.lock() {
            if let Some(_) = registry.get($cmd) {
                return true;
            }
        }
        return false;
    })()};
}

#[event(message)]
async fn on_command(ctx: Context, msg: Message) {
    if !msg.content.starts_with("!") {
        return;
    }
    let cmd_temp: Vec<&str> = msg.content[1..].split(" ").collect();
    let mut old_cmd: String = cmd_temp[0].to_string();
    if !cmd_exists!(&*old_cmd) {
        return;
    }
    let mut cmd: String;
    for i in 1..cmd_temp.len() {
        cmd = cmd_temp[0..=i].join(" ");
        if !cmd_exists!(&*cmd) {
            run_cmd!(&*old_cmd, ctx, msg);
            return;
        }
        old_cmd = cmd;
    }
    run_cmd!(&*old_cmd, ctx, msg);
}

#[event(ready)]
async fn ready(ctx: Context, _data_about_bot: Ready) {
    let test_guild_id = GuildId::new(1355369059037745373);
    println!("e");
    
    let cmds = {
        match TO_REG.lock() {
            Ok(mut registry) => {
                println!("{}", registry.len());
                std::mem::take(&mut *registry) 
            },
            Err(_) => Vec::new(),
        }
    };

    match test_guild_id.set_commands(&ctx.http, cmds).await {
        Err(why) => println!("Error registering commands: {:?}", why),
        Ok(res) => println!("{}", res.len())
    }
    println!("e");
}