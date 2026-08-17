# Brownfield: a contradiction that is really a schema decision

The other examples start from specifications. Most systems do not have any.

This one starts from a running order system, adopted the way a brownfield project actually adopts:
you read the schema, write down what the system already does, and find out what you have. The
contradiction that falls out is not a bug in either subsystem. It is the cost of a decision made in
2016, and it has been invisible ever since because the two halves never used the same word.

The example is in the repository at `examples/legacy-order-system`.

## What already exists

Two tables, written seven years apart by two teams.

```sql title="src/schema/customer_directory.sql"
CREATE TABLE customer_directory (
    customer_id     BIGINT PRIMARY KEY,
    email           TEXT        NOT NULL,
    postal_address  TEXT        NOT NULL,   -- overwritten in place on change
    updated_at      TIMESTAMPTZ NOT NULL,
    erased_at       TIMESTAMPTZ NULL
);
```

```sql title="src/schema/shipment_archive.sql"
CREATE TABLE shipment_archive (
    shipment_id       BIGINT PRIMARY KEY,
    customer_id       BIGINT      NOT NULL REFERENCES customer_directory (customer_id),
    delivery_address  TEXT        NOT NULL,   -- snapshot at dispatch, never updated
    dispatched_at     TIMESTAMPTZ NOT NULL,
    completed_at      TIMESTAMPTZ NULL
);
```

`postal_address` and `delivery_address` hold the same kind of thing: where a named person receives
post. The archive copies rather than references, and that was a reasonable call. A delivery dispute
turns on where the parcel actually went, not on where the customer lives now.

Nobody has ever written that down, and the two columns have different names.

## Writing the vocabulary first

In a brownfield adoption the terms are already decided. The work is finding out what they are.

```toml title=".specify/vocabulary.toml"
[terms.personal-address]
label = "Personal address"
definition = "A postal address identifying where a natural person lives or receives goods, wherever it is stored."
domain = { kind = "entity" }

[terms.customer-directory]
definition = "The table holding one current postal address per customer, overwritten in place on change."
broader = ["personal-address"]
alt_labels = ["postal address"]

[terms.shipment-archive]
definition = "The table holding one address snapshot per dispatched shipment, never updated after dispatch."
broader = ["personal-address"]
alt_labels = ["delivery address"]
```

This is the moment the example turns on. Neither subsystem used the phrase `personal-address`. One
said `postal_address`, the other said `delivery_address`, and because they were different words
nobody ever asked whether they were the same datum.

Declaring both as narrower terms of one concept is not paperwork. It is the assertion that makes
everything afterwards possible.

`ears-sdd vocab-init --all` proposes candidates from existing requirement prose, which is a useful
start on a real codebase. It cannot make this particular judgement for you: deciding that two
differently named columns are the same concept is the work.

## Writing down what the system does

Ten requirements per subsystem, retro-specified from the schema and the code. From the customer
directory, describing the erasure endpoint added in 2024:

> **REQ-004**: When a verified erasure request completes, the system shall delete that customer's
> personal address from every store it controls.

From the shipment archive, describing a retention job that predates it by three years:

> **REQ-002**: The shipment archive shall retain a completed shipment's personal address for seven
> years after completion.

Both are accurate descriptions of what the code does. Both cite a real obligation.

## Each subsystem is fine

```console
$ ears-sdd validate --phase tasks --feature specs/001-customer-directory
EARS/TDD tasks gate: PASS

$ ears-sdd validate --phase tasks --feature specs/002-shipment-archive
EARS/TDD tasks gate: PASS
```

## Together they are not

```console
$ ears-sdd validate --phase tasks --all
EARS/TDD tasks gate: FAIL
Features: 2  Requirements: 20  Errors: 2  Warnings: 0

- MERGE_CONFLICT_UNADJUDICATED [specs/001-customer-directory:REQ-004]
  REQ-004 and REQ-002 contradict each other and nothing says which wins.
  Declare precedence between `honour-erasure-requests` and `provable-delivery-history`.

- MERGE_CONFLICT_UNADJUDICATED [specs/001-customer-directory:REQ-004]
  REQ-004 and REQ-005 contradict each other and nothing says which wins.
```

Two findings, one cause. The witnesses:

```text
erasure-requested = true, order-unfulfilled = false,
shipment-completed = true, shipment-age-years = [0, 6]

erasure-requested = true, order-unfulfilled = false, dispute-open = true
```

A customer with no outstanding order asks to be erased, and has a parcel delivered within the last
seven years. The directory says delete the address. The archive says keep it. Both are quoting a
legal obligation.

The integer term reports as a region, `[0, 6]`, because the model distinguishes ages only at the
constants the guards mention. Every age in that range behaves identically, so the witness names the
range rather than picking a number and implying it mattered.

## What the finding actually tells you

The obvious reading is that the erasure endpoint has a bug: it clears one table and reports success.
That is true, and it is not the finding.

The finding is that `personal-address` has two homes with two lifecycles. The erasure endpoint was
built against the store the privacy team knew about. It would have been just as wrong if it had
cleared both, because the archive's retention rule exists for a reason and clearing it destroys
evidence someone is entitled to.

This is why precedence cannot fix it:

```toml title=".specify/intentions.toml"
# Nothing orders erasure against delivery history, and that is not an omission left for the reader
# to fill in. Neither obligation yields to the other, so any ordering declared here would be a
# fiction that silences the finding.
```

The tool offers to record a decision. Here, honestly recording one is impossible, and that is the
signal. When two requirements contradict and neither goal can yield, the contradiction is not
between the requirements. It is in the design underneath them.

## The resolution is a schema change

Stop storing the same personal datum twice with two lifecycles. The usual shapes:

- Tokenise the archived address. The archive keeps a reference and a non-identifying summary, such
  as the delivery region, adequate for a dispute. Erasure destroys the token and the archive keeps a
  provable record of a delivery it can no longer attribute to a person.
- Make the archive the authority for delivered addresses and the directory the authority for current
  ones, then define erasure over each explicitly rather than as "every store it controls".

Either way the fix is upstream of both specifications. That is the value of the finding: it points
at a schema decision, not at whichever team most recently touched an endpoint.

The third intention in the example is the one neither subsystem was built with:

```toml
[intentions.one-place-per-fact]
statement = "Each fact about a customer has exactly one authoritative home."
```

## What this costs to adopt

Twenty requirements, one vocabulary file, two model files covering five requirements between them.
The other fifteen requirements are reported as unmodelled, because they concern validation, access
scope, and reporting, and cannot contradict anything on the lifecycle axis.

You do not model a brownfield system. You model the axis on which it can be silently wrong, and for
a system holding personal data across two tables, that axis is what survives and for how long.

## Next

- [A needle in 383 requirements](needle-in-a-haystack.md) for the same machinery at scale.
- [Grounding](../concepts/grounding.md) for why the vocabulary layer is the part that made this
  findable.
