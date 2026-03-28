use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TemperaturePayload {
    pub temperature: f64,
    pub unit: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StoredReading {
    pub id: i64,
    pub temperature: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
}

// Shape of the JSON stored in Secrets Manager
#[derive(Debug, Deserialize)]
pub struct DbSecret {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: Option<u16>,
    pub dbname: String,
}