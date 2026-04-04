mod db;
mod handlers;
mod models;

use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client as SecretsClient;
use axum::{
    routing::{get, post},
    Router,
};

use models::DbSecret;
use sqlx::mysql::MySqlConnectOptions;

#[tokio::main]
async fn main() {
    // 1. Load AWS config from environment (IAM role, env vars, ~/.aws/credentials)
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    // 2. Fetch the secret
    let secret_name = std::env::var("DB_SECRET_NAME")
        .expect("DB_SECRET_NAME env var must be set (e.g. prod/temperature-logger/db)");

    let secrets_client = SecretsClient::new(&aws_config);
    let secret_value = secrets_client
        .get_secret_value()
        .secret_id(&secret_name)
        .send()
        .await
        .expect("Failed to fetch secret from Secrets Manager");

    let secret_str = secret_value
        .secret_string()
        .expect("Secret has no string value");

    let secret: DbSecret =
        serde_json::from_str(secret_str).expect("Failed to parse secret JSON");

    // 3. Build the MySQL connection string
    // 3. Build connection options directly (avoids URL parsing issues)
    let opts = MySqlConnectOptions::new()
        .host(&secret.host)
        .port(secret.port)
        .username(&secret.username)
        .password(&secret.password)
        .database(&secret.dbname);

    // 4. Connect and migrate
    let pool = db::create_pool_from_opts(opts).await;
    db::run_migrations(&pool).await;



    // 5. Start the server
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/temperature", post(handlers::post_temperature))
        .route("/temperatures", get(handlers::get_temperatures))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}