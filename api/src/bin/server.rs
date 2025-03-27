use std::{net::SocketAddr, time::Duration};

use api::build_router;
use volo_http::{
    context::ServerContext,
    http::StatusCode,
    server::{layer::TimeoutLayer, Router, Server},
    Address,
};

fn timeout_handler(_: &ServerContext) -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Timeout!\n")
}

#[volo::main]
async fn main() {
    let _logger_guard = common::logger::init_tracing();

    let app = Router::new()
        .merge(build_router())
        .layer(TimeoutLayer::new(Duration::from_secs(1), timeout_handler));

    let addr = "[::]:8080".parse::<SocketAddr>().unwrap();
    let addr = Address::from(addr);

    tracing::info!("Listening on {addr}");

    Server::new(app).run(addr).await.unwrap();
}
