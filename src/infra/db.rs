use sqlx::{Pool, Postgres};

pub async fn get_db_poll() -> Pool<Postgres> {
    let url = std::env::var("DB_URL").unwrap_or_else(|_| "postgresql://root:root@localhost:5432/auth-service-db".into());

    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(10).idle_timeout(std::time::Duration::from_secs(5 * 60))
        .connect(&url).await {
        Ok(conn) => conn,
        Err(_) => {
            panic!("{}", format!("Cannot connect to {}", url).to_string());
        }
    }
}