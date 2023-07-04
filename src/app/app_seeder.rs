use actix_web::{Responder, HttpResponse, get};

use diesel::PgConnection;
use dotenv::dotenv;

use crate::app::db_conn::establish_connection;

use crate::app::app_config::AppConfig;
use crate::app::app_config::app_load_config;

pub struct AppSeeder {
    pub db_conn: PgConnection,
    pub app_config: AppConfig,

}

impl AppSeeder {
    pub fn new() -> Self {
        dotenv().ok();
        let db_conn = establish_connection();
        let app_config=app_load_config();
        return Self { db_conn, app_config }
    }

    pub fn main(self) -> impl Responder {
        let mut response=format!("App Seeder is disabled");
        
        if self.app_config.app_setup_eneabled {
            response=format!("App Seeder is enabled.\n Now running...");
        }

        HttpResponse::Ok().body(response)
    }
}

#[get("/app_seeder/{status}")]
pub async fn app_seeder() -> impl Responder {
    let app_seeder = AppSeeder::new();
    app_seeder.main()
}