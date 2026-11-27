-- One account per network: a keyed hash (HMAC-SHA256) of the address an account was
-- signed up from (IPv4, or the IPv6 /64). Never the raw address. Kept while the account
-- exists, for abuse prevention; see docs/data-inventory.md.
CREATE TABLE signup_networks (
    network_hash BLOB    NOT NULL
                 CONSTRAINT check_signup_networks_network_hash_length
                     CHECK (length(network_hash) = 32),
    account_id   BLOB    NOT NULL
                 CONSTRAINT fk_signup_networks_account_id_accounts
                     REFERENCES accounts (id) ON DELETE CASCADE,
    created_at   INTEGER NOT NULL
                 CONSTRAINT check_signup_networks_created_at_positive CHECK (created_at >= 0),
    CONSTRAINT pk_signup_networks PRIMARY KEY (network_hash)
) STRICT;

CREATE INDEX index_signup_networks_on_account_id ON signup_networks (account_id);
