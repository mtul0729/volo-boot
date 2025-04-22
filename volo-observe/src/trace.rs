use metainfo::Forward;
use metainfo::METAINFO;
use tracing::{info_span, Instrument};
use uuid::Uuid;
use volo::FastStr;
use volo::Layer;
use volo::Service;
use volo::context::Context;

/// A tracing layer that wraps a service and adds distributed tracing capabilities.
///
/// This layer is used to propagate and generate trace and span IDs for distributed tracing.
/// It ensures that each request is instrumented with a unique trace and span ID.
#[derive(Clone)]
pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer {
    type Service = TracingMiddleware<S>;

    fn layer(self, inner: S) -> Self::Service {
        TracingMiddleware { inner }
    }
}

/// A middleware that adds distributed tracing capabilities to a service.
///
/// This struct is used internally by the `TracingLayer` to propagate and generate
/// trace and span IDs for each request.
#[derive(Clone)]
pub struct TracingMiddleware<S> {
    inner: S,
}

#[volo::service]
impl<Cx, Req, S> Service<Cx, Req> for TracingMiddleware<S>
where
    S: Send + 'static + Service<Cx, Req> + Sync,
    Cx: Send + 'static + Context,
    Req: Send + 'static,
{
        async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        // ——— 1. 提取/生成 TraceID & SpanID ———
        let (parent_trace, parent_span) = METAINFO.with(|mi| {
            let mi = mi.borrow();
            let t = mi.get_upstream("trace_id");
            let s = mi.get_upstream("span_id");
            (t, s)
        });
        let trace_id = parent_trace.unwrap_or_else(|| Uuid::new_v4().to_string().into());
        let span_id:FastStr  = Uuid::new_v4().to_string().into();
        METAINFO.with(|mi| {
            let mut mi = mi.borrow_mut();
            mi.set_persistent("trace_id", trace_id.clone());
            mi.set_transient ("span_id",  span_id.clone());
        });

        // ——— 2. 创建并 Instrument Span ———
        let span = info_span!(
            "request",
            trace_id  = %trace_id,
            span_id   = %span_id,
            parent_id = parent_span.as_deref().unwrap_or(""),
            service = %cx.rpc_info().callee().service_name(),
        );
        
        // ——— 3. 执行业务调用并退出 Span ———
        self.inner.call(cx, req).instrument(span).await
    }
}
