// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        business_name -> Text,
        currency -> Text,
        balance -> Numeric,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        key -> Text,
        owner_name -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        from_account -> Nullable<Uuid>,
        to_account -> Nullable<Uuid>,
        amount -> Numeric,
        kind -> Text,
        metadata -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    webhook_events (id) {
        id -> Uuid,
        webhook_id -> Uuid,
        payload -> Jsonb,
        delivered -> Nullable<Bool>,
        attempts -> Nullable<Int4>,
        last_attempt -> Nullable<Timestamptz>,
        next_try -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Uuid,
        account_id -> Uuid,
        url -> Text,
        secret -> Text,
        enabled -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(webhook_events -> webhooks (webhook_id));
diesel::joinable!(webhooks -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    api_keys,
    transactions,
    webhook_events,
    webhooks,
);
