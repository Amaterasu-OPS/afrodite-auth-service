use sqlx::types::chrono;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
pub struct OauthClient {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub secret: String,
    pub urls: Vec<String>,
    pub scopes: Option<Vec<String>>,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}