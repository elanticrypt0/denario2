use actix_web::{Responder, HttpResponse, get};

use diesel::PgConnection;
use dotenv::dotenv;

use crate::app::{db_conn::establish_connection, app_config::{AppConfig, self}};

pub struct CreditsFeature {
    pub db_conn: PgConnection,
    pub app_config: AppConfig,
}

impl CreditsFeature{
    pub fn new() -> Self {
        dotenv().ok();
        let db_conn = establish_connection();
        let app_config = app_config::app_load_config();
       
        return Self { db_conn, app_config }
    }

    pub fn find_all(&self) -> Vec<String> {
        let mut credits: Vec<String> = Vec::new();
        credits.push("Actix Web".to_string());
        credits.push("Diesel".to_string());
        credits.push("PostgreSQL".to_string());
        credits.push("Rust".to_string());
        return credits;
    }

    pub fn find_one(&self, id: i32) -> String {
        let credits = self.find_all();
        return credits[id as usize].clone();
    }

    pub fn create(&self, credit: String) -> String {
        let mut credits = self.find_all();
        credits.push(credit);
        return credits[credits.len() - 1].clone();
    }

    pub fn update(&self, id: i32, credit: String) -> String {
        let mut credits = self.find_all();
        credits[id as usize] = credit;
        return credits[id as usize].clone();
    }

    pub fn delete(&self, id: i32) -> String {
        let mut credits = self.find_all();
        credits.remove(id as usize);
        return credits[id as usize].clone();
    }

}