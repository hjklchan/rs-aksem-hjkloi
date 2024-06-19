use std::env;

use dotenvy::dotenv;

pub fn init() {
    dotenv().ok().expect("failed to load .env file");
}

pub fn get(key: &str) -> String {
    env::var(key).expect(&format!("value of key `{key}` is not present"))
}
