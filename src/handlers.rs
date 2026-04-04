use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use sqlx::MySqlPool;

use crate::models::{StoredReading, TemperaturePayload};

pub async fn root() -> &'static str {
    "Temperature logger is running"
}

pub async fn post_temperature(
    State(pool): State<MySqlPool>,
    Json(payload): Json<TemperaturePayload>,
) -> Result<(StatusCode, Json<StoredReading>), (StatusCode, String)> {
    let unit = payload.unit.unwrap_or_else(|| "C".to_string());
    let timestamp = payload.timestamp.unwrap_or_else(Utc::now);

    let result = sqlx::query(
        "INSERT INTO temperature_readings (temperature, unit, timestamp) VALUES (?, ?, ?)"
    )
        .bind(payload.temperature)
        .bind(&unit)
        .bind(timestamp)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let reading = StoredReading {
        id: result.last_insert_id() as i64,
        temperature: payload.temperature,
        unit,
        timestamp,
    };

    Ok((StatusCode::CREATED, Json(reading)))
}

pub async fn get_temperatures(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<StoredReading>>, (StatusCode, String)> {
    let readings = sqlx::query_as::<_, StoredReading>(
        "SELECT id, temperature, unit, timestamp FROM temperature_readings ORDER BY timestamp DESC"
    )
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(readings))
}