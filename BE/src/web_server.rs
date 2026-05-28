use axum::extract::Json;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Router};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::task;
use tower_http::cors::{Any, CorsLayer};
use crate::simulation::Simulation;
use crate::simulation::config::SimulationConfig;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn run() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/simulate", post(simulate).get(simulate_with_defaults))
        .layer(cors);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind server socket");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn simulate_with_defaults() -> Result<impl IntoResponse, (StatusCode, String)> {
    run_simulation(SimulationConfig::default()).await
}

async fn simulate(Json(request): Json<SimulationConfig>) -> Result<impl IntoResponse, (StatusCode, String)> {
    run_simulation(request).await
}

async fn run_simulation(
    request: SimulationConfig,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    task::spawn_blocking(move || {
        let mut simulation = Simulation::new(
            request
        );
        simulation.simulate();
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            simulation.get_recording_as_json(),
        )
    })
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
}
