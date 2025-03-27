pub mod app_config;
pub mod svc_discover;
pub mod consts;

pub mod controller{
    pub mod user_controller;
}

use volo_http::Extension;
use volo_http::server::route::{get, Router};
use user::user::UserServiceClient;

/// 这个结构体里面放每个rpc的客户端,目前只放了一个user的
#[derive(Clone)]
pub struct ServiceContext {
    pub rpc_cli_user: Option<UserServiceClient>
}


pub fn build_router(cxt: ServiceContext) -> Router {
    Router::new()
        .route("/", get(|| async { "Ok." }))
        .route("/user", get(controller::user_controller::get_user))
        .layer(Extension(cxt))
}
