use sqlx::MySqlPool;

pub async fn create_pool(connection_string: &str) -> MySqlPool{
    MySqlPool::connect(connection_string)
        .await
        .expect("Failed to connect to MySQL")
}
