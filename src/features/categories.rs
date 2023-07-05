use actix_web::{Responder, HttpResponse, get,post,patch,delete};

use crate::core::feature_core::FeatureCore;

#[get("/categories")]
pub async fn find_all() -> impl Responder {
    let fcore=FeatureCore::load();

    let app_name=fcore.app_config.app_name.clone();

    let response = app_name;

    HttpResponse::Ok().json({
        response
    })
}

#[get("/categories/{id}")]
pub async fn find_one() -> impl Responder {
    let fcore=FeatureCore::load();
    let credits = find_all();
    return credits[id as usize].clone();
    
    HttpResponse::Ok().json({
        credits
    })
}

#[post("/categories")]
pub async fn create(credit: String) -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits.push(credit);
    // return credits[credits.len() - 1].clone();

    HttpResponse::Ok().json({
        credits
    })
}

#[patch("/categories/{id}")]
pub async fn update() -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits[id as usize] = credit;
    // return credits[id as usize].clone();

    HttpResponse::Ok().json({
        credits
    })
}

#[delete("/categories/{id}")]
pub async fn delete() -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits.remove(id as usize);
    // return credits[id as usize].clone();

    HttpResponse::Ok().json({
        credits
    })
}