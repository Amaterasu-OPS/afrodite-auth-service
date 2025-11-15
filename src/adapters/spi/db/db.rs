use crate::application::spi::db::{DBFactory, DBInterface};

pub struct DBAdapter;

impl DBAdapter {
    pub async fn get_db_connection<T>() -> Result<T, sqlx::Error>
    where
        T: DBInterface<> + DBFactory,
    {
        T::get().await
    }
}