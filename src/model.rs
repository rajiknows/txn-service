use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub business_name: String,
    pub currency: String,
    pub balance: Decimal,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateAccountPayload {
    pub business_name: String,
    pub currency: Option<String>,
}