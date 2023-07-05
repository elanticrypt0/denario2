use actix_web::{Responder, HttpResponse, get};

use crate::core::feature_core::FeatureCore;
use crate::core::helpers::fsmaker::{create_file_in_current_folder,mk_dir_in_current_folder};

#[get("/app_run_setup")]
pub async fn app_run_setup() -> impl Responder {
    let fcore=FeatureCore::load();
    let mut response=format!("App Setup is disabled");
    
    if fcore.app_config.app_setup_eneabled {
        create_folders();
        plaint_seeds();

        response=format!("App Setup is enabled.\n Now running...");
    }
    HttpResponse::Ok().body(response)
}

fn create_folders(){
    // self.mk_dir_in_current_folder("config".to_string());
    mk_dir_in_current_folder("public".to_string());
    mk_dir_in_current_folder("public/assets".to_string());
    mk_dir_in_current_folder("uploads".to_string());
    mk_dir_in_current_folder("seeds".to_string());
}

fn plaint_seeds(){
    create_file_in_current_folder("seeds/users.json".to_string());
    create_file_in_current_folder("seeds/posts.json".to_string());
}