use actix_web::{Responder, HttpResponse, get,post,patch,delete, web::Data};
use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool};

use crate::core::feature_core::FeatureCore;
use crate::models::categories_model::Category;

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
pub async fn find_one(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let conn=pool.get().expect("Cant connect to DB");

    
    
    HttpResponse::Ok().json({
        credits
    })
}

// #[post("/categories")]
// pub async fn create(req_body: String, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {

//     let fcore=FeatureCore::load();
    
    

//     HttpResponse::Ok().json({
//         credits
//     })
// }

// #[patch("/categories/{id}")]
// pub async fn update(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
//     let fcore=FeatureCore::load();
//     // let mut credits = find_all();
//     // credits[id as usize] = credit;
//     // return credits[id as usize].clone();

//     HttpResponse::Ok().json({
//         credits
//     })
// }

// #[delete("/categories/{id}")]
// pub async fn delete(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
//     let fcore=FeatureCore::load();
//     // let mut credits = find_all();
//     // credits.remove(id as usize);
//     // return credits[id as usize].clone();

//     HttpResponse::Ok().json({
//         credits
//     })
// }