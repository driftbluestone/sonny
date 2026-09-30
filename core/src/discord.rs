use serenity::model::prelude::*;
use serenity::prelude::*;
use registry::REGISTRY;

macro_rules! run {
    ($event_type:literal, $($args:expr),*) => {{
        if let Ok(registry) = REGISTRY.lock() {
            if let Some(callbacks) = registry.get($event_type) {
                for callback in callbacks {
                    let mut callback_args: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
                    $(
                        let arg_arc = std::sync::Arc::new(tokio::sync::RwLock::new($args.clone()));
                        callback_args.push(Box::new(arg_arc) as Box<dyn std::any::Any + Send + Sync>);
                    )*
                    
                    tokio::spawn(callback(callback_args)); 
                }
            }
        }
    
    }};
}

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn command_permissions_update(&self, ctx: Context, permission: CommandPermissions) {
        run!("command_permissions_update", ctx, permission);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        run!("message", ctx, msg);
    }
}

