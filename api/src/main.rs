use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create a channel for graceful shutdown
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    // Define API routes
    let app = Router::new()
        .route("/plans/diagnose", get(diagnose_plan))
        .with_state(shutdown_tx);

    // Bind to port (configurable via env for K8s)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Starting API server on {}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_rx.recv().await;
            info!("Shutting down API server");
        })
        .await
        .expect("Server failed to start");
}

// Handler for /plans/diagnose
async fn diagnose_plan() -> &'static str {
    // Placeholder: Implement PlanSense integration
    "Diagnosing query plan..."
}