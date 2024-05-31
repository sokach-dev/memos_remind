use serde::Deserialize;

use macros::config;

#[config("CONFIG_URL")]
#[derive(Debug, Deserialize)]
pub struct AConfig {
    pub name: String,
    pub age: u32,
}

impl AConfig {
    pub fn say(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

#[tokio::main]
async fn main() {
    let cg = AConfig {
        name: "Alice".to_string(),
        age: 20,
    };
    println!("{:?}", cg);
    println!("{:?}", cg.say());
    let c = get_global_config().await;
    println!("{:?}", c);
}

// CONFIG_URL=./app.toml cargo run --example demo
