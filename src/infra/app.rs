use actix_web::{App, HttpServer, web};
use crate::adapters::api;
use crate::adapters::spi::db::db::DBAdapter;
use crate::adapters::spi::db::postgres_db::PostgresDB;
use crate::infra::redis::get_redis_pool;

pub async fn start_app() -> std::io::Result<()> {
    let psql = DBAdapter::get_db_connection::<PostgresDB>().await.expect("Failed to connect to postgres database");
    
    let redis_pool_data = web::Data::new(get_redis_pool());
    let postgres_poll_data = web::Data::new(psql);

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/api/v1")
            .service(api::health::router::health_router())
            .service(api::auth::router::auth_router())
        )
        .app_data(redis_pool_data.clone())
        .app_data(postgres_poll_data.clone())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}