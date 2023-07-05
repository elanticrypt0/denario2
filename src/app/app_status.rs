use actix_web::{Responder, HttpResponse, get};

use crate::core::feature_core::FeatureCore;


#[get("/app_check_status")]
pub async fn app_check_status() -> impl Responder {
    let fcore=FeatureCore::load();
    
    let response=format!("App check running on: {} mode",fcore.app_config.app_env.to_uppercase());
    
    HttpResponse::Ok().json({
        response
    })
}