use std::collections::HashMap;
use volo_http::{http::StatusCode, server::extract::Query, json::Json};
use volo_http::server::IntoResponse;


pub async fn get_user(
    Query(param): Query<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    param
        .get("id")
        .map(|id| Json(id.clone()))
        .ok_or(StatusCode::BAD_REQUEST)
}
