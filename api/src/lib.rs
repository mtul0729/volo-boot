pub mod controller{
    pub mod user_controller;
}
use volo_http::server::route::{get, Router};


pub fn build_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Ok." }))
        .route("/user", get(controller::user_controller::get_user))

}
