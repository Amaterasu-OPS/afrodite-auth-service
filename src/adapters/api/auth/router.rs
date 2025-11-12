use actix_web::{HttpResponse, Responder, Scope, get, post, web};
use crate::application::use_cases::auth::par::ParUseCase;
use crate::application::use_cases::use_case::UseCase;
use crate::dto::auth::par::{request::ParRequest};

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(par_handler)
        .service(authorize_handler)
        .service(token_handler)
}

#[post("/par")]
async fn par_handler(
    data: web::Form<ParRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
    db_pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> impl Responder {
    let case = ParUseCase {
        redis_pool: redis_pool.into_inner(),
        db_pool: db_pool.into_inner(),
    };

    let result = case.handle(data.into_inner());

    match result.await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/authorize")]
async fn authorize_handler() -> impl Responder {
    HttpResponse::Ok().body("auth authorize endpoint")
}

#[post("/token")]
async fn token_handler() -> impl Responder {
    HttpResponse::Ok().body("auth token endpoint")
}