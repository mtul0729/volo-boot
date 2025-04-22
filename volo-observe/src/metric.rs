use volo_http::http::{StatusCode, Uri};
use volo_http::request::ServerRequest;
use volo_http::response::ServerResponse;
use volo_http::server::IntoResponse;
use volo_http::{context::ServerContext, server::middleware::Next};
use std::time::Instant;

/// Tracks metrics for HTTP requests and responses.
///
/// This function is a middleware that measures the latency of HTTP requests,
/// counts the total number of requests, and records the duration of each request.
/// It also captures metadata such as the HTTP method, path, and response status.
///
/// # Arguments
/// * `uri` - The URI of the incoming request.
/// * `cx` - The server context, which includes metadata and other information.
/// * `req` - The incoming server request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// A `Result` containing the server response or an HTTP status code in case of an error.
///
/// # Metrics
/// * `http_requests_total` - A counter for the total number of HTTP requests.
/// * `http_requests_duration_seconds` - A histogram for the duration of HTTP requests in seconds.
pub async fn track_metrics(
    uri: Uri,
    cx: &mut ServerContext,
    req: ServerRequest,
    next: Next,
) -> Result<ServerResponse, StatusCode> {
    let start = Instant::now();

    let path = uri.path();
    let method = req.method().clone();

    let response = next.run(cx, req).await;

    let latency = start.elapsed().as_secs_f64();

    let Ok(r) = response else {
        return Ok(response.into_response());
    };

    let labels = [
        ("method", method.to_string()),
        ("path", path.to_string()),
        ("status", r.status().as_u16().to_string()),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    Ok(r)
}
