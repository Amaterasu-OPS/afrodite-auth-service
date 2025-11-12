use deadpool_redis::Config;

pub fn get_redis_pool() -> deadpool_redis::Pool {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let cfg = Config::from_url(redis_url);


    cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).expect("Cannot create Redis pool")
}