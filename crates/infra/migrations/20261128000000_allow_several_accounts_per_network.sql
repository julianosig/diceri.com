-- no-transaction
-- Several accounts per network (a configurable number): the network is no longer the key.
-- Rebuilt with SQLite's 12-step procedure; nothing references this table.
PRAGMA foreign_keys = OFF;
BEGIN IMMEDIATE;

CREATE TABLE new_signup_networks (
    account_id   BLOB    NOT NULL
                 CONSTRAINT fk_signup_networks_account_id_accounts
                     REFERENCES accounts (id) ON DELETE CASCADE,
    network_hash BLOB    NOT NULL
                 CONSTRAINT check_signup_networks_network_hash_length
                     CHECK (length(network_hash) = 32),
    created_at   INTEGER NOT NULL
                 CONSTRAINT check_signup_networks_created_at_positive CHECK (created_at >= 0),
    CONSTRAINT pk_signup_networks PRIMARY KEY (account_id)
) STRICT;

INSERT INTO new_signup_networks (account_id, network_hash, created_at)
    SELECT account_id, network_hash, created_at FROM signup_networks;

DROP TABLE signup_networks;
ALTER TABLE new_signup_networks RENAME TO signup_networks;

-- Counting a network's accounts at sign-up.
CREATE INDEX index_signup_networks_on_network_hash ON signup_networks (network_hash);

PRAGMA foreign_key_check;
COMMIT;
PRAGMA foreign_keys = ON;
