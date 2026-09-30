use std::{any::Any, collections::HashMap, future::Future, pin::Pin, sync::{LazyLock, Mutex}};
pub static REGISTRY: LazyLock<Mutex<HashMap<&'static str, Vec<fn(Vec<Box<dyn Any + Send + Sync>>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register(
    key: &'static str, 
    f: fn(Vec<Box<dyn Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>
) {
    if let Ok(mut registry) = REGISTRY.lock() {
        registry.entry(key).or_insert_with(Vec::new).push(f);
    }
}