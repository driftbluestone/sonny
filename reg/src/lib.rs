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
        let mut cmd_types: Vec<(String, String, bool)> = Vec::new();
        let cmd_opts = $command.data.options.clone();

        if let Ok(registry) = ACMD_REG.lock() {
            if let Some(_) = registry.get($cmd) {}
            else {
                return;
            }
        }
        if let Ok(registry) = ACMD_TYPE_REG.lock() {
            if let Some(v) = registry.get($cmd) {
                cmd_types = v.clone();
            }
        }

        let mut callback_args: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
        callback_args.push(Box::new($ctx.clone()) as Box<dyn std::any::Any + Send + Sync>);
        callback_args.push(Box::new($command.clone()) as Box<dyn std::any::Any + Send + Sync>);

        macro_rules! push_arg {
            ($val:expr, $is_opt:expr) => {{
                if !$is_opt {
                    callback_args.push(Box::new(Some($val)) as Box<dyn std::any::Any + Send + Sync>);
                } else {
                    callback_args.push(Box::new($val) as Box<dyn std::any::Any + Send + Sync>);
                }
            }};
        }

        macro_rules! push_none {
            ($t:ty) => {
                callback_args.push(Box::new(None::<$t>) as Box<dyn std::any::Any + Send + Sync>)
            };
        }
        
        for i in 0..cmd_types.len() {
            //let arg_type: &(String, String, bool) = &cmd_types[i];
            let (arg_name, arg_type, is_req) = &cmd_types[i];
            //let arg_type = &arg_type.1;
            let option_type = &cmd_opts.iter().find(|opt| opt.name == *arg_name);
            if let Some(opt_type) = *option_type {
                match opt_type.value {
                    CommandDataOptionValue::Boolean(v) => {
                        callback_args.push(Box::new(v) as Box<dyn std::any::Any + Send + Sync>);
                    }
                    CommandDataOptionValue::Integer(v) => {
                        match &**arg_type {
                            "u8"  => push_arg!(v as u8, *is_req),
                            "u16" => push_arg!(v as u16, *is_req),
                            "u32" => push_arg!(v as u32, *is_req),
                            "u64" => push_arg!(v as u64, *is_req),
                            "u128"=> push_arg!(v as u128, *is_req),
                            "i8"  => push_arg!(v as i8, *is_req),
                            "i16" => push_arg!(v as i16, *is_req),
                            "i32" => push_arg!(v as i32, *is_req),
                            "i64" => push_arg!(v as i64, *is_req),
                            "i128"=> push_arg!(v as i128, *is_req),
                            _ => {}
                        };
                    }
                    CommandDataOptionValue::Number(v) => {
                        match &**arg_type {
                            // f16 and f128 are not yet fully implemented
                            "f32" => push_arg!(v as f32, *is_req),
                            "f64" => push_arg!(v as f64, *is_req),
                            _ => {}
                        }
                    }
                    CommandDataOptionValue::String(ref v) => {
                        push_arg!(v.to_string(), *is_req)
                    }
                    CommandDataOptionValue::Attachment(v) => {
                        push_arg!(v, *is_req);
                    }
                    CommandDataOptionValue::Channel(v) => {
                        match &**arg_type {
                            "ChannelId" => push_arg!(v, *is_req),
                            "Channel" => {
                                if let Ok(c) = v.to_channel(&$ctx.http).await {
                                    push_arg!(c, *is_req);
                                }
                            }
                            _ => {}
                        }
                    }
                    CommandDataOptionValue::Mentionable(v) => {
                        push_arg!(v, *is_req)
                    }
                    CommandDataOptionValue::Role(v) => {
                        match &**arg_type {
                            "RoleId" => push_arg!(v, *is_req),
                            "Role" => {
                                let role_opt = $ctx.cache.guild($command.guild_id.expect("Couldn't get Guild ID. Are you in DMs?")).and_then(|guild| {
                                    guild.roles.get(&v).cloned() // .cloned() turns &Role into Role
                                });
                                if let Some(role) = role_opt {
                                    push_arg!(role, *is_req)
                                }
                            }
                            _ => {}
                        }
                    }
                    CommandDataOptionValue::User(v) => {
                        match &**arg_type {
                            "UserId" => push_arg!(v, *is_req),
                            "User" => {
                                if let Ok(c) = v.to_user(&$ctx.http).await {
                                    push_arg!(c, *is_req)
                                }
                            },
                            "Member" => {
                                if let Ok(c) = $command.guild_id.expect("Couldn't get Guild ID. Are you in DMs?").member(&$ctx.http, v).await {
                                    push_arg!(c, *is_req)
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else {
                match &**arg_type {
                    "bool" | "Boolean" => push_none!(bool),
                    "u8" => push_none!(u8),
                    "u16" => push_none!(u16),
                    "u32" => push_none!(u32),
                    "u64" => push_none!(u64),
                    "u128" => push_none!(u128),
                    "i8" => push_none!(i8),
                    "i16" => push_none!(i16),
                    "i32" => push_none!(i32),
                    "i64" => push_none!(i64),
                    "i128" => push_none!(i128),
                    "f32" => push_none!(f32),
                    "f64" => push_none!(f64),
                    "String" => push_none!(String),
                    "ChannelId" => push_none!(serenity::model::id::ChannelId),
                    "Channel" => push_none!(serenity::model::channel::Channel),
                    "RoleId" => push_none!(serenity::model::id::RoleId),
                    "Role" => push_none!(serenity::model::guild::Role),
                    "UserId" => push_none!(serenity::model::id::UserId),
                    "User" => push_none!(serenity::model::user::User),
                    "Member" => push_none!(serenity::model::guild::Member),
                    _ => {}
                }
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
async fn on_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let name = &command.data.name;
        run_acmd!(name.as_str(), ctx, command);
        // let v = command.data.options.iter().find()
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
async fn ready(ctx: Context, data_about_bot: Ready) {
    run_event!("before_acmd_register", &ctx, &data_about_bot);
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