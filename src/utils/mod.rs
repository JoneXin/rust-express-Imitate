pub mod tool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub app_config: AppConfig,
}


#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub request_body_size: usize,
    pub port: i32,
}
