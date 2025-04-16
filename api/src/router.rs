use std::future::ready;
use volo_http::{Extension, Router};
use volo_http::server::middleware;
use volo_http::server::route::get;
use crate::{controller, ServiceContext};
use crate::prometheus::{setup_metrics_recorder, track_metrics};

/// 构建路由
pub fn build_router(cxt: ServiceContext) -> Router {

    let record_handler = setup_metrics_recorder();

    Router::new()
        .route("/metrics", get(move || ready(record_handler.render())))
        .route("/user/query-one", get(controller::user_controller::get_user))
        .route("/order/query-one", get(controller::order_controller::get_order))
        .layer(middleware::from_fn(track_metrics))
        .layer(Extension(cxt))
}


