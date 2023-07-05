use dotenvy::dotenv;
use std::env;
// use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::*;

pub fn postgres_establish_connection() -> Pool<ConnectionManager<PgConnection>>{
    let db_url=load_config("POSGRES_URL");
    return get_pool_pgconnection(db_url);
}

fn load_config(connection_name: &str) -> String{
    dotenv().ok();
    let db_url=env::var(connection_name).expect("DATABASE_URL not found in config");
    return db_url
}

// Conección a PosgresSQL
fn get_pool_pgconnection(db_url: String) -> Pool<ConnectionManager<PgConnection>>{
    let manager=ConnectionManager::<PgConnection>::new(db_url);
    let pool=Pool::builder().build(manager).expect("Cant connect to DB");

    return pool;
}