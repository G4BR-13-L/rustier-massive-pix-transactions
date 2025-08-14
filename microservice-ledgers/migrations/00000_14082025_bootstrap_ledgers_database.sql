-- ============================
-- ledger DB
-- ============================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ENUMS
CREATE TYPE entry_type AS ENUM ('DEBIT', 'CREDIT');

-- TABLE: ledger_entries
CREATE TABLE ledger_entries (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transfer_id     UUID NOT NULL,
    account_id      UUID NOT NULL,
    entry_type      entry_type NOT NULL,
    amount          NUMERIC(19,4) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ledger_entries_transfer_id ON ledger_entries(transfer_id);
CREATE INDEX idx_ledger_entries_account_id ON ledger_entries(account_id);

-- TABLE: statements
CREATE TABLE statements (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id       UUID NOT NULL,
    period_start     TIMESTAMPTZ NOT NULL,
    period_end       TIMESTAMPTZ NOT NULL,
    opening_balance  NUMERIC(19,4) NOT NULL,
    closing_balance  NUMERIC(19,4) NOT NULL,
    generated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_statements_account_id ON statements(account_id);
CREATE INDEX idx_statements_period ON statements(account_id, period_start, period_end);
