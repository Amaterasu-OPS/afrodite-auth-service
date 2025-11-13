use sqlx::{Database, Encode, FromRow, Type};

pub trait DBInterface: Send + Sync {
    type DB: Database;
    type Err;

    type T;

    async fn connect(user: String, password: String, host: String, port: String, db_name: String) -> Self::T where Self: Sized;
    async fn new() -> Self where Self: Sized;

    async fn select_many<R, U>(&self, query: &str, binds: Vec<U>) -> Result<Vec<R>, Self::Err>
    where
        R: for<'r> FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin + Sync,
        U: for<'q> Encode<'q, Self::DB> + Type<Self::DB> + Send + Sync + 'static;

    async fn select_one<R, U>(&self, query: &str, binds: Vec<U>) -> Result<R, Self::Err>
    where
        R: for<'r> FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin + Sync,
        U: for<'q> Encode<'q, Self::DB> + Type<Self::DB> + Send + Sync + 'static;

    async fn execute(&self, query: &str) -> Result<<Self::DB as Database>::QueryResult, Self::Err>;
}

pub trait DBFactory: Sized {
    async fn get() -> Result<Self, sqlx::Error>;
}