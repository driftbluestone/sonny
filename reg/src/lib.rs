pub use pma::*;
pub use registry::*;

#[doc(hidden)]
pub mod __private {
    pub use ctor;
    pub use syn;
    pub use registry;
}

extern crate self as reg;

use serenity::all::{CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub trait CommandInteractionExt {
    fn response(&self, ctx: &Context, content: &str, ephemeral: bool) ->
        impl std::future::Future<Output = Result<serenity::model::prelude::Message, SerenityError>> + Send;
}

impl CommandInteractionExt for CommandInteraction {
    async fn response(&self, ctx: &Context, content: &str, ephemeral: bool) -> Result<Message, SerenityError> {
        let message = CreateInteractionResponseMessage::new()
            .content(content).ephemeral(ephemeral);
        let response = CreateInteractionResponse::Message(message);
        self.create_response(&ctx.http, response).await?;
        self.get_response(&ctx.http).await
    }
}

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

macro_rules! run_acmd {
    ($cmd:expr, $ctx:expr, $command:expr) => {{
        let mut cmd_types: Vec<String> = Vec::new();
        let cmd_opts = $command.data.options.clone();

        if let Ok(registry) = ACMD_REG.lock() {
            if let Some(_) = registry.get($cmd) {}
            else {
                return;
            }
            if let Ok(registry) = ACMD_TYPE_REG.lock() {
                if let Some(v) = registry.get($cmd) {
                    cmd_types = v.clone();
                }
            }
        }

        let mut callback_args: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
        callback_args.push(Box::new($ctx.clone()) as Box<dyn std::any::Any + Send + Sync>);
        callback_args.push(Box::new($command.clone()) as Box<dyn std::any::Any + Send + Sync>);
        
        
        for i in 0..cmd_opts.len() {
            let arg_type: &String = &cmd_types[i];
            let option_type = &cmd_opts[i];
            match option_type.value {
                CommandDataOptionValue::Boolean(v) => {
                    callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>);
                }
                CommandDataOptionValue::Integer(v) => {
                    match &**arg_type {
                        "u8" => callback_args.push(Box::new(v as u8) as Box<dyn std::any::Any + Send + Sync>),
                        "u16" => callback_args.push(Box::new(v as u16) as Box<dyn std::any::Any + Send + Sync>),
                        "u32" => callback_args.push(Box::new(v as u32) as Box<dyn std::any::Any + Send + Sync>),
                        "u64" => callback_args.push(Box::new(v as u64) as Box<dyn std::any::Any + Send + Sync>),
                        "u128" => callback_args.push(Box::new(v as u128) as Box<dyn std::any::Any + Send + Sync>),
                        "i8" => callback_args.push(Box::new(v as i8) as Box<dyn std::any::Any + Send + Sync>),
                        "i16" => callback_args.push(Box::new(v as i16) as Box<dyn std::any::Any + Send + Sync>),
                        "i32" => callback_args.push(Box::new(v as i32) as Box<dyn std::any::Any + Send + Sync>),
                        "i64" => callback_args.push(Box::new(v as i64) as Box<dyn std::any::Any + Send + Sync>),
                        "i128" => callback_args.push(Box::new(v as i128) as Box<dyn std::any::Any + Send + Sync>),
                        _ => {}
                    };
                }
                CommandDataOptionValue::Number(v) => {
                    match &**arg_type {
                        // f16 and f128 are not yet fully implemented, uncomment on impl
                        //"f16" => callback_args.push(Box::new(v as f16) as Box<dyn std::any::Any + Send + Sync>),
                        "f32" => callback_args.push(Box::new(v as f32) as Box<dyn std::any::Any + Send + Sync>),
                        "f64" => callback_args.push(Box::new(v as f64) as Box<dyn std::any::Any + Send + Sync>),
                        //"f128" => callback_args.push(Box::new(v as f128) as Box<dyn std::any::Any + Send + Sync>),
                        _ => {}
                    }
                }
                CommandDataOptionValue::String(ref v) => {
                    callback_args.push(Box::new(v.to_string()) as Box<dyn std::any::Any + Send + Sync>);
                }
                CommandDataOptionValue::Attachment(v) => {
                    callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>);
                }
                CommandDataOptionValue::Channel(v) => {
                    match &**arg_type {
                        "ChannelId" => callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>),
                        "Channel" => {
                            if let Ok(c) = v.to_channel(&$ctx.http).await {
                                callback_args.push(Box::new(c) as Box<dyn std::any::Any + Send + Sync>)
                            }
                        }
                        _ => {}
                    }
                }
                CommandDataOptionValue::Mentionable(v) => {
                    callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>);
                }
                CommandDataOptionValue::Role(v) => {
                    match &**arg_type {
                        "RoleId" => callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>),
                        "Role" => {
                            let role_opt = $ctx.cache.guild($command.guild_id.expect("Couldn't get Guild ID. Are you in DMs?")).and_then(|guild| {
                                guild.roles.get(&v).cloned() // .cloned() turns &Role into Role
                            });
                            if let Some(role) = role_opt {
                                callback_args.push(Box::new(role) as Box<dyn std::any::Any + Send + Sync>)
                            }
                        }
                        _ => {}
                    }
                }
                CommandDataOptionValue::User(v) => {
                    match &**arg_type {
                        "UserId" => callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>),
                        "User" => {
                            if let Ok(c) = v.to_user(&$ctx.http).await {
                                callback_args.push(Box::new(c) as Box<dyn std::any::Any + Send + Sync>)
                            }
                        },
                        "Member" => {
                            if let Ok(c) = $command.guild_id.expect("Couldn't get Guild ID. Are you in DMs?").member(&$ctx.http, v).await {
                                callback_args.push(Box::new(c) as Box<dyn std::any::Any + Send + Sync>)
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            
        }
        if let Ok(registry) = ACMD_REG.lock() {
            if let Some(callback) = registry.get($cmd) {
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

#[event(interaction_create)]
async fn test(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let name = &command.data.name;
        run_acmd!(name.as_str(), ctx, command);
    }
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
    
    let cmds = {
        match TO_REG.lock() {
            Ok(mut registry) => {
                std::mem::take(&mut *registry) 
            },
            Err(_) => Vec::new(),
        }
    };

    match test_guild_id.set_commands(&ctx.http, cmds).await {
        Err(why) => println!("Error registering commands: {:?}", why),
        Ok(res) => println!("Registered {} commands.", res.len())
    }
}