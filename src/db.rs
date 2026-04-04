use sqlx::mysql::{MySqlConnectOptions, MySqlPool};

pub async fn create_pool_from_opts(opts: MySqlConnectOptions) -> MySqlPool {
    MySqlPool::connect_with(opts)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to MySQL: {}", e))
}

pub async fn run_migrations(pool: &MySqlPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS temperature_readings (
            id          BIGINT AUTO_INCREMENT PRIMARY KEY,
            temperature DOUBLE      NOT NULL,
            unit        VARCHAR(10) NOT NULL DEFAULT 'C',
            timestamp   DATETIME(3) NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await
        .expect("Failed to run migrations");
}