use actix_web::{Responder, HttpResponse, get};

use crate::app::app_config::AppConfig;
use crate::app::app_config::app_load_config;


struct AppStatus {
    pub app_config: AppConfig,
}

impl AppStatus {
    pub fn new() -> Self{
        let app_config=app_load_config();
        return Self { app_config }
    }
}

#[get("/app_check_status")]
pub async fn app_check_status() -> impl Responder {
    let app_status = AppStatus::new();
    
    let response=format!("App check running on: {} mode",app_status.app_config.app_env.to_uppercase());
    
    HttpResponse::Ok().json({
        response
    })
}