use axum::{http::StatusCode, Router, routing::get, ServiceExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(||async { "Hello, world!"}))
        .route("/health", get(|| async { StatusCode::NO_CONTENT }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
