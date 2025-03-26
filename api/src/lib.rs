use volo_http::server::route::{get, Router};

async fn index_handler() -> &'static str {
    "It Works!\n"
}

pub fn example_router() -> Router {
    Router::new().route("/", get(index_handler))
}
