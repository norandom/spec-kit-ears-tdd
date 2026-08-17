-- Shipment archive. Added 2016, before the customer directory existed.
--
-- The delivery address is copied here at dispatch rather than referenced, because at the time there
-- was nothing stable to reference and because a dispute needs the address as it stood on the day,
-- not as it stands now. That decision is defensible and it is also the whole problem: the same
-- personal datum now lives in two tables with two different lifecycles.

CREATE TABLE shipment_archive (
    shipment_id       BIGINT PRIMARY KEY,
    customer_id       BIGINT      NOT NULL REFERENCES customer_directory (customer_id),

    -- Snapshot of the address the parcel was actually sent to. Never updated after dispatch.
    delivery_address  TEXT        NOT NULL,

    dispatched_at     TIMESTAMPTZ NOT NULL,
    completed_at      TIMESTAMPTZ NULL
);

CREATE INDEX shipment_archive_customer ON shipment_archive (customer_id);
CREATE INDEX shipment_archive_completed ON shipment_archive (completed_at);
