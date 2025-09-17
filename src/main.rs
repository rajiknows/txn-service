use crate::db::get_pool;
use dotenvy::dotenv;
use poem::{EndpointExt, Route, Server, get, handler, listener::TcpListener, post};

mod db;
mod handlers;
mod model;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = get_pool(&db_url).await;

    let app = Route::new()
        .at("/health", get(health))
        .at("/accounts", post(handlers::accounts::create_account))
        .at("/accounts/:id", get(handlers::accounts::get_account))
        .data(pool);

    println!("Server running at http://localhost:8080");
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}

#[handler]
async fn health() -> &'static str {
    "ok"
}
