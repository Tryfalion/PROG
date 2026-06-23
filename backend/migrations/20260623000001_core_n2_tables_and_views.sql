-- SQLite und PostgreSQL konforme Migration für das N2 System basierend auf Spec 003.

CREATE TABLE invoices (
    id UUID PRIMARY KEY,
    invoice_number VARCHAR NOT NULL UNIQUE,
    issue_date DATE NOT NULL,
    due_date DATE NOT NULL,
    amount NUMERIC(18,6) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE bank_transactions (
    id UUID PRIMARY KEY,
    booking_date DATE NOT NULL,
    value_date DATE NOT NULL,
    amount NUMERIC(18,6) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    reference_text TEXT NOT NULL,
    counterparty_name VARCHAR,
    counterparty_iban VARCHAR,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE allocations (
    id UUID PRIMARY KEY,
    transaction_id UUID NOT NULL REFERENCES bank_transactions(id),
    invoice_id UUID NOT NULL REFERENCES invoices(id),
    allocated_amount NUMERIC(18,6) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Derived Status View (N2 Approach)
CREATE VIEW invoice_statuses_view AS
SELECT 
    i.id AS invoice_id,
    i.invoice_number,
    i.amount AS invoice_amount,
    COALESCE(SUM(a.allocated_amount), 0) AS total_allocated,
    CASE
        WHEN COALESCE(SUM(a.allocated_amount), 0) >= i.amount AND COALESCE(SUM(a.allocated_amount), 0) <= i.amount THEN 'Paid'
        WHEN COALESCE(SUM(a.allocated_amount), 0) > i.amount THEN 'Overpaid'
        ELSE 'Open'
    END AS status
FROM 
    invoices i
LEFT JOIN 
    allocations a ON i.id = a.invoice_id
GROUP BY 
    i.id;
