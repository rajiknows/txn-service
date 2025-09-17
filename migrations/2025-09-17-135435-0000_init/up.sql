-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE api_keys (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  key text NOT NULL UNIQUE,
  owner_name text,
  created_at timestamptz DEFAULT now()
);

CREATE TABLE accounts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  business_name text NOT NULL,
  currency text NOT NULL DEFAULT 'USD',
  balance numeric(20,4) NOT NULL DEFAULT 0,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);

CREATE TABLE transactions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  from_account uuid NULL REFERENCES accounts(id),
  to_account uuid NULL REFERENCES accounts(id),
  amount numeric(20,4) NOT NULL CHECK (amount > 0),
  kind text NOT NULL,
  metadata jsonb DEFAULT '{}'::jsonb,
  created_at timestamptz DEFAULT now()
);

CREATE TABLE webhooks (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES accounts(id),
  url text NOT NULL,
  secret text NOT NULL,
  enabled boolean DEFAULT true,
  created_at timestamptz DEFAULT now()
);

CREATE TABLE webhook_events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  webhook_id uuid NOT NULL REFERENCES webhooks(id),
  payload jsonb NOT NULL,
  delivered boolean DEFAULT false,
  attempts int DEFAULT 0,
  last_attempt timestamptz NULL,
  next_try timestamptz DEFAULT now(),
  created_at timestamptz DEFAULT now()
);

CREATE INDEX idx_webhook_events_next_try ON webhook_events(next_try);
