use crate::prometheus::setup_metrics_recorder;
use crate::{controller, ServiceContext};
use std::future::ready;
use volo_http::{
    response::ServerResponse,
    server::{middleware, route::get, IntoResponse},
    Extension, Router,
};
use volo_observe::metric::track_metrics;

/// 构建路由
pub fn build_router(cxt: ServiceContext) -> Router {
    let record_handler = setup_metrics_recorder();

    Router::new()
        .route("/metrics", get(move || ready(record_handler.render())))
        .route(
            "/user/query-one",
            get(controller::user_controller::get_user),
        )
        .route(
            "/order/query-one",
            get(controller::order_controller::get_order),
        )
        .layer(middleware::from_fn(track_metrics))
        .layer(middleware::map_response(headers_map_response))
        .layer(Extension(cxt))
}

async fn headers_map_response(response: ServerResponse) -> impl IntoResponse {
    (
        [
            ("Access-Control-Allow-Origin", "*"),
            ("Access-Control-Allow-Headers", "*"),
            ("Access-Control-Allow-Method", "*"),
        ],
        response,
    )
}
