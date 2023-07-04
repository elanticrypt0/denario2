use std::{env, fs};

use actix_web::{Responder, HttpResponse, get};

use crate::app::app_config::AppConfig;
use crate::app::app_config::app_load_config;

use diesel::PgConnection;

use crate::app::db_conn::establish_connection;

pub struct AppSetup {
    pub app_config: AppConfig,
    pub db_conn: PgConnection,
}

impl AppSetup {
    pub fn new() -> Self {
        
        let app_config=app_load_config();
        let db_conn = establish_connection();
        return Self { app_config,db_conn }
    }
    
    pub fn main(self) -> impl Responder {
        let mut response=format!("App Setup is disabled");
        
        if self.app_config.app_setup_eneabled {
            self.create_folders();
            self.plaint_seeds();

            response=format!("App Setup is enabled.\n Now running...");
        }
        HttpResponse::Ok().body(response)
    }
    
    
    fn mk_dir_in_current_folder(&self,new_dir:String){
        let mut path = env::current_dir().unwrap();
        path.push(new_dir);
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
    }
    
    fn create_file_in_current_folder(&self,new_file:String){
        let mut path = env::current_dir().unwrap();
        path.push(new_file);
        if !path.exists() {
            fs::write(path, "").unwrap();
        }
    }

    pub fn create_folders(&self){
        // self.mk_dir_in_current_folder("config".to_string());
        self.mk_dir_in_current_folder("public".to_string());
        self.mk_dir_in_current_folder("public/assets".to_string());
        self.mk_dir_in_current_folder("uploads".to_string());
        self.mk_dir_in_current_folder("seeds".to_string());
    }

    pub fn plaint_seeds(&self){        // seeds
        self.create_file_in_current_folder("seeds/users.json".to_string());
        self.create_file_in_current_folder("seeds/posts.json".to_string());
    }
}


#[get("/app_run_setup")]
pub async fn app_run_setup() -> impl Responder {
    let app_setup = AppSetup::new();
    app_setup.main()
}