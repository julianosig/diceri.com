-- Each account's own voice recording (ADR 0015), the one exception to "no audio in the
-- database": AES-256-GCM ciphertext under a server key that is never stored here, bound
-- to the account id. Kept while the account exists or until the person deletes it.
CREATE TABLE voice_samples (
    account_id   BLOB    NOT NULL
                 CONSTRAINT fk_voice_samples_account_id_accounts
                     REFERENCES accounts (id) ON DELETE CASCADE,
    content_type TEXT    NOT NULL
                 CONSTRAINT check_voice_samples_content_type_length
                     CHECK (length(content_type) BETWEEN 1 AND 100),
    nonce        BLOB    NOT NULL
                 CONSTRAINT check_voice_samples_nonce_length CHECK (length(nonce) = 12),
    -- At most 5 MiB of audio plus the 16-byte authentication tag.
    ciphertext   BLOB    NOT NULL
                 CONSTRAINT check_voice_samples_ciphertext_length
                     CHECK (length(ciphertext) BETWEEN 17 AND 5242896),
    recorded_at  INTEGER NOT NULL
                 CONSTRAINT check_voice_samples_recorded_at_positive CHECK (recorded_at >= 0),
    CONSTRAINT pk_voice_samples PRIMARY KEY (account_id)
) STRICT;
