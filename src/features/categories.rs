use actix_web::{Responder, HttpResponse, get};

use diesel::PgConnection;
use dotenv::dotenv;

use crate::{app::{db_conn::establish_connection, app_config::{AppConfig, self}}, schema::categories, models::categories_model::Category};

pub struct CategoriesFeature {
    pub db_conn: PgConnection,
    pub app_config: AppConfig,

}
impl CategoriesFeature{
    pub fn new() -> Self {
        dotenv().ok();
        let mut db_conn =establish_connection();
        let app_config = app_config::app_load_config();
        return Self { db_conn, app_config }
    }

    pub fn find_all(&self) -> Vec<Category> {
        
        let categories= Category::find_all(&self.db_conn);
        return categories;
        
    }

    // pub fn find_one(&self, id: i32) -> String {
    //     let categories = self.find_all();
    //     return categories[id as usize].clone();
    // }

    // pub fn create(&self, category: String) -> String {
    //     let mut categories = self.find_all();
    //     categories.push(category);
    //     return categories[categories.len() - 1].clone();
    // }

    // pub fn update(&self, id: i32, category: String) -> String {
    //     let mut categories = self.find_all();
    //     categories[id as usize] = category;
    //     return categories[id as usize].clone();
    // }

    // pub fn delete(&self, id: i32) -> String {
    //     let mut categories = self.find_all();
    //     categories.remove(id as usize);
    //     return categories[id as usize].clone();
    // }
}

//  Routes

#[get("/categories")]
pub async fn get_categories() -> impl Responder {
    let categories= CategoriesFeature::new();
    let results = categories.find_all();

    let response = HttpResponse::Ok().json(results);
    return response;
}