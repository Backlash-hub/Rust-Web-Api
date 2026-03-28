mod models;
mod db;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// The JSON shape Postman (or your MCU) will POST
#[derive(Debug, Deserialize, Serialize, Clone)]
struct TemperatureReading {
    temperature: f64,       // e.g. 23.5
    unit: Option<String>,   // e.g. "C" or "F" — optional
    timestamp: Option<DateTime<Utc>>, // optional; we'll fill it in if missing
}

// What we store internally (always has a timestamp)
#[derive(Debug, Serialize, Clone)]
struct StoredReading {
    temperature: f64,
    unit: String,
    timestamp: DateTime<Utc>,
}

// Shared state type alias for readability
type SharedState = Arc<Mutex<Vec<StoredReading>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/temperature", post(post_temperature))
        .route("/temperatures", get(get_temperatures))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// GET / — health check
async fn root() -> &'static str {
    "Temperature logger is running"
}

// POST /temperature — accept a reading
async fn post_temperature(
    State(state): State<SharedState>,
    Json(payload): Json<TemperatureReading>,
) -> (StatusCode, Json<StoredReading>) {
    let reading = StoredReading {
        temperature: payload.temperature,
        unit: payload.unit.unwrap_or_else(|| "C".to_string()),
        timestamp: payload.timestamp.unwrap_or_else(Utc::now),
    };

    let stored = reading.clone();
    state.lock().unwrap().push(reading);

    (StatusCode::CREATED, Json(stored))
}

// GET /temperatures — return all readings
async fn get_temperatures(
    State(state): State<SharedState>,
) -> Json<Vec<StoredReading>> {
    let readings = state.lock().unwrap().clone();
    Json(readings)
}