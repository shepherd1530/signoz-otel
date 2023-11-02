use axum::{routing::get, Router};
use otel_tracer_signoz::logger::{get_subscriber, init_subscriber};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let subscriber = get_subscriber("signoz".into(), "info".into());

    let _log_guard = init_subscriber(subscriber);

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::info!("hello from /");

    "Hello, World!"
}
