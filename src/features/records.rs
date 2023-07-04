use actix_web::{Responder, HttpResponse, get};

use diesel::PgConnection;
use dotenv::dotenv;

use crate::app::{db_conn::establish_connection, app_config::{AppConfig, self}};

pub struct RecordsFeature {
    pub db_conn: PgConnection,
    pub app_config: AppConfig,

}

impl RecordsFeature{
    pub fn new() -> Self {
        dotenv().ok();
        let db_conn = establish_connection();
        let app_config = app_config::app_load_config();
        return Self { db_conn, app_config }
    }
}