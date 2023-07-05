use diesel::PgConnection;
use dotenv::dotenv;

use crate::core::app_core_config::{AppCoreConfig,app_load_config};
// use crate::core::db_manager_core::connection;

pub struct FeatureCore {
    pub db_conn: PgConnection,
    pub app_config: AppCoreConfig,
}

impl FeatureCore{
    pub fn load() -> Self {
        dotenv().ok();
        // let mut db_conn =connection();
        let app_config = app_load_config();
        return Self { db_conn,app_config }
    }
}