use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize, Selectable)]
#[table_name = "accounts"]
#[diesel(check_for_backend(Pg))]
pub struct Account {
    pub id: Uuid,
    pub business_name: String,
    pub currency: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateAccountPayload {
    pub business_name: String,
    pub currency: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub business_name: String,
    pub currency: String,
}

#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: Uuid,
    pub from_account: Option<Uuid>,
    pub to_account: Option<Uuid>,
    pub amount: Decimal,
    pub kind: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub from_account: Option<Uuid>,
    pub to_account: Option<Uuid>,
    pub amount: Decimal,
    pub kind: String,
    pub metadata: serde_json::Value,
}

#[derive(Queryable, Identifiable)]
#[table_name = "api_keys"]
pub struct ApiKey {
    pub id: Uuid,
    pub key: String,
    pub owner_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub id: Uuid,
    pub account_id: Uuid,
    pub url: String,
    pub secret: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
