-- ============================
-- transfers DB
-- ============================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ENUMS
CREATE TYPE transfer_status AS ENUM ('PENDING', 'POSTED', 'REVERSED', 'FAILED');

-- TABLE: transfers
CREATE TABLE transfers (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    external_id         VARCHAR(255) UNIQUE,
    debit_account_id    UUID NOT NULL,
    credit_account_id   UUID NOT NULL,
    amount              NUMERIC(19,4) NOT NULL,
    currency            CHAR(3) NOT NULL, -- ISO 4217
    status              transfer_status NOT NULL,
    narrative           TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    posted_at           TIMESTAMPTZ,
    reversed_at         TIMESTAMPTZ
);

-- Indexes para performance
CREATE INDEX idx_transfers_debit_account ON transfers(debit_account_id);
CREATE INDEX idx_transfers_credit_account ON transfers(credit_account_id);
CREATE INDEX idx_transfers_status ON transfers(status);
