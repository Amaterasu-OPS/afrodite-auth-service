use actix_web::{App, HttpServer, web};

use crate::adapters::api;
use crate::infra::db::get_db_poll;
use crate::infra::redis::get_redis_pool;

pub async fn start_app() -> std::io::Result<()> {
    let redis_pool_data = web::Data::new(get_redis_pool());
    let postgres_poll_data = web::Data::new(get_db_poll().await);

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