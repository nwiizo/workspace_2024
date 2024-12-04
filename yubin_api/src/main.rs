use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::info;

use yubin_api::{
    metrics::setup_metrics,
    routes::{health_check, lookup_by_postal_code, search_by_address},
};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yubin_api=debug,tower_http=debug".into()),
        )
        .init();

    // Initialize metrics
    setup_metrics();

    // Setup request tracing
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // Build router with all middleware
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/postal/:code", get(lookup_by_postal_code))
        .route("/address/search", post(search_by_address))
        .layer(ServiceBuilder::new().layer(trace_layer).layer(cors));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);

    // Start metrics server
    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    info!("Metrics server listening on {}", metrics_addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
