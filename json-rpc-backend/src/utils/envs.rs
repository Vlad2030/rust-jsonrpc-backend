use dotenv::dotenv;
use std::env;

pub fn get_env(variable: &'static str) -> Option<String> {
    dotenv().ok();
    let var = std::env::var(variable);
    match var {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub fn update_env(
    key: &'static str,
    value: String,
) -> () {
    env::set_var(key, value)
}
