use actix_web::{Responder, HttpResponse, get};

use crate::core::feature_core::FeatureCore;

#[get("/categories")]
pub async fn get_categories() -> impl Responder {
    let fcore=FeatureCore::load();

    let app_name=fcore.app_config.app_name.clone();

    let response = HttpResponse::Ok().json(app_name);
    return response;
}