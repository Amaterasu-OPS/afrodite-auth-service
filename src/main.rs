mod infra;
mod adapters;
mod dto;
mod application;
mod domain;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    infra::app::start_app().await
}