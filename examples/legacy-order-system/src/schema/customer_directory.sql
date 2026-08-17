-- Customer directory. Added 2019 with the account self-service work, and extended in 2024 when the
-- erasure endpoint was built.
--
-- This is the table the privacy team considers the address of record.

CREATE TABLE customer_directory (
    customer_id     BIGINT PRIMARY KEY,
    email           TEXT        NOT NULL,

    -- The customer's current postal address. Overwritten in place on change, so the directory
    -- holds one address per customer and no history.
    postal_address  TEXT        NOT NULL,

    updated_at      TIMESTAMPTZ NOT NULL,
    erased_at       TIMESTAMPTZ NULL
);

CREATE INDEX customer_directory_email ON customer_directory (email);
