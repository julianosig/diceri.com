-- The voice clone kept at the voice provider for each account (ADR 0017): created once from
-- the kept recording and reused in every call, replaced when the person records again, and
-- deleted with "Delete my voice" or the account. Only the provider's voice id is stored.
CREATE TABLE account_voices (
    account_id BLOB    NOT NULL
               CONSTRAINT fk_account_voices_account_id_accounts
                   REFERENCES accounts (id) ON DELETE CASCADE,
    provider   TEXT    NOT NULL
               CONSTRAINT check_account_voices_provider_enum
                   CHECK (provider IN ('elevenlabs', 'inworld')),
    voice_id   TEXT    NOT NULL
               CONSTRAINT check_account_voices_voice_id_length
                   CHECK (length(voice_id) BETWEEN 1 AND 200),
    created_at INTEGER NOT NULL
               CONSTRAINT check_account_voices_created_at_positive CHECK (created_at >= 0),
    CONSTRAINT pk_account_voices PRIMARY KEY (account_id)
) STRICT;
