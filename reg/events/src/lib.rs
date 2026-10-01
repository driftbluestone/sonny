use serenity::model::prelude::*;
use serenity::prelude::*;
use registry::CMD_REG;
use pma::event;

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