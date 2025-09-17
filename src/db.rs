use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

pub type PgPool = Pool<AsyncPgConnection>;

pub async fn get_pool(database_url: &str) -> PgPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder().build(config).await.unwrap()
}
