use crate::db::get_pool;
use crate::model::{Account, CreateAccountPayload};
use poem::{
    web::{Json, Path},
    Result, handler,
};
use uuid::Uuid;

#[handler]
pub async fn create_account(
    Json(payload): Json<CreateAccountPayload>,
) -> Result<Json<Account>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = get_pool(&db_url).await;

    let currency = payload.currency.unwrap_or_else(|| "USD".to_string());

    let acc = sqlx::query_as::<_, Account>(
        "INSERT INTO accounts (business_name, currency) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.business_name)
    .bind(currency)
    .fetch_one(&pool)
    .await
    .map_err(poem::error::InternalServerError)?;

    Ok(Json(acc))
}

#[handler]
pub async fn get_account(Path(acc_id): Path<Uuid>) -> Result<Json<Account>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = get_pool(&db_url).await;

    let acc = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1")
        .bind(acc_id)
        .fetch_one(&pool)
        .await
        .map_err(poem::error::NotFound)?;

    Ok(Json(acc))
}
