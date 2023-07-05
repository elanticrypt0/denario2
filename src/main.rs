use actix_cors::Cors;
use actix_web::{middleware::Logger, http };
use actix_files as fs;
use actix_web::{App, HttpServer };

// Database
mod schema;

// Application
use crate::core::app_core_config::{app_load_config};
mod app;
mod core;
mod features;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config=app_load_config();

    let server=format!("{}:{}",config.app_server_host,config.app_server_port);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    app_print_welcome(&config.app_server_host,config.app_server_port,&config.app_name,&config.app_version);

    HttpServer::new(move || {
        // CORS
        let cors = Cors::default()
              .allowed_origin("http://localhost:8080")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"8080")
              })
              .allowed_methods(vec!["GET", "POST","PATCH","DELETE"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
            .wrap(cors)
            // app_setup
            .service(app::app_setup::app_run_setup)
            .service(app::app_status::app_check_status)
            // STATIC
            .service(fs::Files::new("/","./public").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(server)?
    .run()
    .await
}

fn app_print_welcome(server_host:&String,server_port:u16,app_name:&str,app_version:&str){

    println!("🦀-----------------------------------------------------------🦀");
    println!("                  🪙  {} [{}]",app_name,app_version);
    println!("   🚀 Server started successfully at http://{}:{}",server_host,server_port);
    println!("   🔗 View in webbrowser at http://{}:{}/",server_host,server_port);
    println!("🦀-----------------------------------------------------------🦀");

}