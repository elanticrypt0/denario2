use dotenvy::dotenv;


use crate::core::app_core_config::{AppCoreConfig,app_load_config};

pub struct FeatureCore {
    pub app_config: AppCoreConfig,
}

impl FeatureCore{
    pub fn load() -> Self {
        dotenv().ok();
        let app_config = app_load_config();
        return Self { app_config }
    }
}