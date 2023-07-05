use dotenv::dotenv;
use std::{env};

#[derive(Debug, Clone)]
pub struct AppCoreConfig{
    pub app_setup_eneabled: bool,
    pub app_debug_mode: bool,
    pub app_server_host: String,
    pub app_server_port: u16,
    pub app_url: String,
    pub app_name: String,
    pub app_version: String,
    pub app_env: String,
}

impl AppCoreConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let app_setup_eneabled = env::var("APP_SETUP_ENABLED").expect("APP_SETUP must be set") == "true";
        let app_debug_mode = env::var("APP_DEBUG_MODE").expect("APP_DEBUG_MODE must be set") == "true";
        let app_server_host = env::var("APP_SERVER_HOST").expect("APP_HOST must be set");
        let app_server_port = env::var("APP_SERVER_PORT").expect("APP_PORT must be set").parse::<u16>().unwrap();
        let app_name = env!("CARGO_PKG_NAME").to_string();
        let app_version = env!("CARGO_PKG_VERSION").to_string();
        let app_env = env::var("APP_ENV").expect("APP_ENV must be set");

        let app_url = format!("{}:{}", app_server_host, app_server_port);
        
        return Self { app_setup_eneabled, app_debug_mode, app_server_host, app_server_port, app_name, app_version,app_env, app_url }
    }
}

pub fn app_load_config() -> AppCoreConfig {
    return AppCoreConfig::new();
}