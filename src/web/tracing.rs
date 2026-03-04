//! Custom HTTP request tracing layer.
//!
//! This module provides request logging with the following format:
//! `[<request_id>] <method> <path> <status> <latency>ms <client_ip>`
//!
//! It also adds the `X-Request-ID` header to responses.

#![forbid(unsafe_code)]

use axum::{body::Body, extract::Request, http::header::HeaderName, response::Response};
use std::task::{Context, Poll};
use tower::{Layer, Service, ServiceExt};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Clone)]
pub struct RequestLoggingLayer;

impl<S> Layer<S> for RequestLoggingLayer {
    type Service = RequestLoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggingService { inner }
    }
}

#[derive(Clone)]
pub struct RequestLoggingService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestLoggingService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn Send + std::future::Future<Output = Result<Self::Response, Self::Error>>>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let method = request.method().clone();
        let uri = request.uri().clone();
        let client_ip = extract_client_ip(&request);
        let start = std::time::Instant::now();

        let mut request = request;
        request.extensions_mut().insert(request_id.clone());

        let inner = self.inner.clone();

        Box::pin(async move {
            let mut response: Response<Body> = inner.oneshot(request).await?;

            let latency = start.elapsed();
            let status = response.status();

            response.headers_mut().insert(
                HeaderName::from_static(REQUEST_ID_HEADER),
                request_id.parse().unwrap(),
            );

            tracing::info!(
                target: "hello-rust",
                "[{}] {} {} {} {}ms {}",
                request_id,
                status.as_u16(),
                method,
                uri,
                latency.as_millis(),
                client_ip,
            );

            Ok(response)
        })
    }
}

fn extract_client_ip(request: &Request) -> String {
    request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
        })
        .unwrap_or("-")
        .to_string()
}
