use std::time::Instant;
use uuid::Uuid;
use volo::Layer;
use volo::Service;
use volo::context::Context;
use tracing::{debug, error, info};

/// A logging layer that wraps a service and logs request and response details.
/// This layer is used to track the lifecycle of requests, including their parameters,
/// responses, and execution time.
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

        fn layer(self, inner: S) -> Self::Service {
        LoggingService { inner }
    }
}

/// A service wrapper that logs request and response details.
///
/// This struct is used internally by the `LoggingLayer` to provide logging functionality.
#[derive(Clone)]
pub struct LoggingService<S> {
    inner: S,
}

#[volo::service]
impl<Cx, Req, S> Service<Cx, Req> for LoggingService<S>
where
    S: Send + 'static + Service<Cx, Req> + Sync,
    Cx: Send + 'static + Context,
    Req: Send + 'static + std::fmt::Debug,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
{
        async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let request_id = Uuid::new_v4().to_string();
        let start_time = Instant::now();

        let params = format!("{:?}", req);
        info!(
            target: "volo_boot_rpc",
            "Request started: request_id={}, service={}, params={}, timestamp={}",
            request_id,
            cx.rpc_info().callee().service_name(),
            params,
            chrono::Utc::now().to_rfc3339(),
        );

        let result = self.inner.call(cx, req).await;

        let duration_ms = start_time.elapsed().as_millis();

        match &result {
            Ok(resp) => {
                let response = format!("{:?}", resp);
                debug!(
                    target: "volo_boot_rpc",
                    "Request succeeded: request_id={}, response={}, duration_ms={}, timestamp={}",
                    request_id,
                    response,
                    duration_ms,
                    chrono::Utc::now().to_rfc3339(),
                );
            }
            Err(err) => {
                error!(
                    target: "volo_boot_rpc",
                    "Request failed: request_id={}, error={:?}, duration_ms={}, timestamp={}",
                    request_id,
                    err,
                    duration_ms,
                    chrono::Utc::now().to_rfc3339(),
                );
            }
        }

        result
    }
}