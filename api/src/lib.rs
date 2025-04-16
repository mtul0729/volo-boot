pub mod app_config;
pub mod svc_discover;
pub mod consts;
pub mod router;
pub mod prometheus;

pub mod controller;

use volo_http::Extension;
use volo_http::server::route::{get, Router};
use user::user::UserServiceClient;
use order::order::OrderServiceClient;

/// 这个结构体里面放每个rpc的客户端,目前只放了一个user的
#[derive(Clone)]
pub struct ServiceContext {
    pub rpc_cli_user: Option<UserServiceClient>,
    pub rpc_cli_order: Option<OrderServiceClient>
}


