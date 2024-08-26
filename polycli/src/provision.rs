use std::{convert::Infallible, fs::OpenOptions, future::Future, path::PathBuf, pin::Pin, task::Poll, time::Duration, io::Write};
use axum::{body::Body, http::{HeaderValue, Method, Request, Response, StatusCode}, RequestExt, Router};

use tower::Service;
use tower_http::{services::ServeDir, trace:: TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};


pub async fn run_provision(endpoint: String, filepath: String) -> anyhow::Result<()> {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "polycli=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let put_handle = PutFallback::new(filepath.clone());
    let serve_dir = ServeDir::new(filepath)
    .fallback(put_handle)
    .call_fallback_on_method_not_allowed(true);

    // why does tracing require SO MUCH CODE
    let route = Router::new()
        .nest_service("/", serve_dir)
        .layer(
            TraceLayer::new_for_http()
                .on_response(|response: &Response<_>, _latency: Duration, _span: &Span| {
                    tracing::info!(target: "response", code=response.status().as_str())
                })
            .on_request(
                |request: &Request<_>, _span: &Span| {
                    let placeholder = &HeaderValue::from_static("");
                    let headers = request.headers().get("user-agent").unwrap_or(placeholder);
                    tracing::info!(target: "request",
                    user_agent = ?headers,
                    );
                },
            )
            .make_span_with(|request: &Request<_>| {
                let matched_path = request.uri();
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    path=%matched_path,
                )
            })
        );

    // actually do server things
    let listener = tokio::net::TcpListener::bind(endpoint)
    .await?;

    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, route).await?;

    Ok(())

}


/// PutFallback is a simple web server to handle the PUT requests that the Polycom provisioner uses for logs.
/// This is needed because ServeDir will only implement GET and HEAD, so we have to do PUT ourselves.
#[derive(Debug, Clone, Default)]
pub struct PutFallback{
    root: PathBuf
}

impl PutFallback {
    pub fn new(path: String) -> Self {
        Self {root:PathBuf::from(path)  }
    }

    fn handle_put(&self, path: String, body: String) -> StatusCode {
        // have to do this cursed thing because the implementation of PathBuf.join() is fundamentally broken:
        // https://github.com/rust-lang/rust/issues/16507
        let final_str = format!("{}/{}", self.root.to_string_lossy(), path); 
        let path = PathBuf::from(final_str);

        tracing::debug!(target: "put", path = path.to_string_lossy().to_string());

        if ! path.starts_with(self.root.as_os_str()) {
            return StatusCode::FORBIDDEN
        }

        let mut out = match OpenOptions::new().append(true).create(true).open(path) {
            Ok(h) => h,
            Err(err) => {
                tracing::error!(target: "put", mode="open", error = err.to_string());
                return StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        match write!(out, "{}", body) {
            Ok(_) => {},
            Err(err) => {
                tracing::error!(target: "put", mode="write", error = err.to_string());
                return StatusCode::INTERNAL_SERVER_ERROR
            }
        }

        tracing::debug!(target: "put", "wrote file");

        StatusCode::OK
    }
}

impl Service<Request<Body>> for PutFallback {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
            // we only deal with the PUT requests that the ServeDir can't handle here
            let adapter = self.clone();
            let fut = async move {
                let code = match *req.method() {
                    // fallback called because file doesn't exist
                    Method::HEAD  | Method::GET => {
                      StatusCode::NOT_FOUND
                    },
                    Method::PUT => {
                        adapter.handle_put(req.uri().path().to_string(),  req.extract().await.unwrap())
                    }
                    _ => { StatusCode::METHOD_NOT_ALLOWED}
                };

                let body = Response::builder().status(code).body(Body::empty()).unwrap();
                Ok(body)
            };
            Box::pin(fut)


            //std::future::ready(Ok(body))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn  test_server() {
        run_provision("0.0.0.0:8000".to_string(), "/home/alexk/Documents/polycom/firmware/UC_Software_6_4_6_release_sig_split".to_string()).await.unwrap();
    }

}