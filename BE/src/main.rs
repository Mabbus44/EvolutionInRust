mod simulation;

use axum::extract::Json;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::task;
use tower_http::cors::{Any, CorsLayer};
use simulation::Simulation;
use simulation::map::MapConfig;
use simulation::generation::GenerationConfig;
use simulation::mutation::MutationConfig;

#[derive(Deserialize)]
struct SimulationRequest {
    map: MapRequest,
    generation: GenerationRequest,
    mutation: MutationRequest,
}

#[derive(Deserialize)]
struct MapRequest {
    carnivore_count: u32,
    herbivore_count: u32,
    grass_count: u32,
    neuron_count: usize,
    neuron_layer_count: usize,
    sense_radius: usize,
    carnivore_max_energy: u32,
    herbivore_max_energy: u32,
    size_x: usize,
    size_y: usize,
    record: bool,
}

#[derive(Deserialize)]
struct GenerationRequest {
    max_generation_count: u32,
    max_ticks_per_generation: i32,
    carnivore_count: i32,
    herbivore_count: i32,
    grass_count: i32,
    best_carnivore_count: u32,
    best_herbivore_count: u32,
    all_entities_must_be_under_min_levels: bool,
}

#[derive(Deserialize)]
struct MutationRequest {
    mutation_chance: f64,
    max_mutation_amount: f64,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

impl Default for SimulationRequest {
    fn default() -> Self {
        Self {
            map: MapRequest {
                carnivore_count: 5,
                herbivore_count: 15,
                grass_count: 100,
                neuron_count: 10,
                neuron_layer_count: 2,
                sense_radius: 4,
                carnivore_max_energy: 100,
                herbivore_max_energy: 50,
                size_x: 200,
                size_y: 100,
                record: true,
            },
            generation: GenerationRequest {
                max_generation_count: 50,
                max_ticks_per_generation: 1000,
                carnivore_count: 2,
                herbivore_count: 2,
                grass_count: -1,
                best_carnivore_count: 2,
                best_herbivore_count: 2,
                all_entities_must_be_under_min_levels: true,
            },
            mutation: MutationRequest {
                mutation_chance: 0.01,
                max_mutation_amount: 0.3,
            },
        }
    }
}

impl From<MapRequest> for MapConfig {
    fn from(value: MapRequest) -> Self {
        Self {
            carnivore_count: value.carnivore_count,
            herbivore_count: value.herbivore_count,
            grass_count: value.grass_count,
            neuron_count: value.neuron_count,
            neuron_layer_count: value.neuron_layer_count,
            sense_radius: value.sense_radius,
            carnivore_max_energy: value.carnivore_max_energy,
            herbivore_max_energy: value.herbivore_max_energy,
            size_x: value.size_x,
            size_y: value.size_y,
            record: value.record,
        }
    }
}

impl From<GenerationRequest> for GenerationConfig {
    fn from(value: GenerationRequest) -> Self {
        Self {
            max_generation_count: value.max_generation_count,
            max_ticks_per_generation: value.max_ticks_per_generation,
            carnivore_count: value.carnivore_count,
            herbivore_count: value.herbivore_count,
            grass_count: value.grass_count,
            best_carnivore_count: value.best_carnivore_count,
            best_herbivore_count: value.best_herbivore_count,
            all_entities_must_be_under_min_levels: value.all_entities_must_be_under_min_levels,
        }
    }
}

impl From<MutationRequest> for MutationConfig {
    fn from(value: MutationRequest) -> Self {
        Self {
            mutation_chance: value.mutation_chance,
            max_mutation_amount: value.max_mutation_amount,
        }
    }
}

#[tokio::main]
async fn main() {
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
    run_simulation(SimulationRequest::default()).await
}

async fn simulate(Json(request): Json<SimulationRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    run_simulation(request).await
}

async fn run_simulation(
    request: SimulationRequest,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    task::spawn_blocking(move || {
        let mut simulation = Simulation::new(
            request.map.into(),
            request.generation.into(),
            request.mutation.into(),
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
