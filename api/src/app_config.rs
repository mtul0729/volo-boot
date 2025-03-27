use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub port: u32,
    pub sd: ServerDiscover,
    // 订阅服务列表
    pub subscribe_service: Vec<String>
}

/// 服务发现配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerDiscover {
    pub nacos: NacosConfig
}

/// nacos
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NacosConfig {
    pub server_addr: String,
    pub namespace: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub service_name: String
}