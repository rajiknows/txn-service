use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn get_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap()
}