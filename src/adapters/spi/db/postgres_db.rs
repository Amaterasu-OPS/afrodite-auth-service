use sqlx::{Database, Encode, Error, FromRow, Pool, Postgres, Type};
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::query::QueryAs;
use crate::application::spi::db::{DBFactory, DBInterface};

#[derive(Clone, Debug)]
pub struct PostgresDB {
    pool: Pool<Postgres>,
}

impl DBFactory for PostgresDB {
    async fn get() -> Result<Self, sqlx::Error> {
        Ok(PostgresDB::new().await)
    }
}

impl DBInterface for PostgresDB {
    type DB = Postgres;
    type Err = Error;
    type T = Pool<Postgres>;

    async fn connect(user: String, password: String, host: String, port: String, db_name: String) -> Pool<Postgres> {
        let url = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db_name);

        let conn = match sqlx::postgres::PgPoolOptions::new()
            .max_connections(10).idle_timeout(std::time::Duration::from_secs(5 * 60))
            .connect(&url).await {
            Ok(conn) => conn,
            Err(_) => {
                panic!("{}", format!("Cannot connect to {}", url).to_string());
            }
        };

        conn
    }

    async fn new() -> Self {
        PostgresDB {
            pool: PostgresDB::connect(
                std::env::var("DB_USER").expect("DB_USER environment variable not set"),
                std::env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set"),
                std::env::var("DB_HOST").expect("DB_HOST environment variable not set"),
                std::env::var("DB_PORT").expect("DB_PORT environment variable not set"),
                std::env::var("DB_DB").expect("DB_DB environment variable not set"),
            ).await
        }
    }

    async fn select_many<R, U>(&self, query: &str, binds: Vec<U>) -> Result<Vec<R>, Error>
    where
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin + Sync,
        U: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send + Sync + 'static,
    {
        self.select(query, binds).fetch_all(&self.pool).await
    }

    async fn select_one<R, U>(&self, query: &str, binds: Vec<U>) -> Result<R, Error>
    where
        R: for<'r> FromRow<'r, PgRow> + Send + Unpin + Sync,
        U: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send + Sync + 'static,
    {
        self.select(query, binds).fetch_one(&self.pool).await
    }

    async fn execute(&self, query: &str) -> Result<PgQueryResult, Error> {
        sqlx::query(&query).execute((&self.pool)).await
    }
}

impl PostgresDB {
    fn select<'q, T, U>(&self, query: &'q str, binds: Vec<U>) -> QueryAs<'q, Postgres, T, <Postgres as Database>::Arguments<'q>>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin + Sync,
        U: for<'a> Encode<'a, Postgres> + Type<Postgres> + Send + Sync + 'static,
    {
        let mut sql = sqlx::query_as::<_, T>(&query);

        for i in binds {
            sql = sql.bind(i);
        }

        sql
    }
}