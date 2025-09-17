use crate::db::PgPool;
use crate::model::{Account, CreateAccountPayload, NewAccount};
use crate::schema::accounts::dsl::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poem::{
    Result, handler,
    web::{Data, Json, Path},
};
use uuid::uuid;

#[handler]
pub async fn create_account(
    Data(pool): Data<&PgPool>,
    Json(payload): Json<CreateAccountPayload>,
) -> Result<Json<Account>> {
    let mut conn = pool.get().await.map_err(poem::error::InternalServerError)?;

    let new_acc = NewAccount {
        business_name: payload.business_name,
        currency: payload.currency.unwrap_or_else(|| "USD".to_string()),
    };

    let acc = diesel::insert_into(accounts)
        .values(&new_acc)
        .returning(Account::as_select())
        .get_result::<Account>(&mut conn)
        .await
        .map_err(poem::error::InternalServerError)?;

    Ok(Json(acc))
}

#[handler]
pub async fn get_account(
    Data(pool): Data<&PgPool>,
    Path(acc_id): Path<Uuid>,
) -> Result<Json<Account>> {
    let mut conn = pool.get().await.map_err(poem::error::InternalServerError)?;
    let acc = accounts
        .find(acc_id)
        .first::<Account>(&mut conn)
        .await
        .map_err(poem::error::NotFound)?;
    Ok(Json(acc))
}
