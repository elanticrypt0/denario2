use actix_web::{Responder, HttpResponse, get};

use crate::core::feature_core::FeatureCore;

#[get("/app_seeder/{status}")]
pub async fn app_seeder() -> impl Responder {
    let fcore=FeatureCore::load();
    let mut response=format!("App Seeder is disabled");
        
    if fcore.app_config.app_setup_eneabled {
        response=format!("App Seeder is enabled.\n Now running...");
    }

    HttpResponse::Ok().body(response)
}