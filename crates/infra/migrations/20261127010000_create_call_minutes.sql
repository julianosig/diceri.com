-- Call minutes per account and UTC day, so one account cannot use everyone's free
-- capacity. Charged to the room's owner, one row per account and day, counted only while
-- someone is in the call. Holds no audio, transcript or call content.
CREATE TABLE call_minutes (
    account_id BLOB    NOT NULL
               CONSTRAINT fk_call_minutes_account_id_accounts
                   REFERENCES accounts (id) ON DELETE CASCADE,
    -- Days since the Unix epoch, UTC.
    utc_day    INTEGER NOT NULL
               CONSTRAINT check_call_minutes_utc_day_positive CHECK (utc_day >= 0),
    minutes    INTEGER NOT NULL
               CONSTRAINT check_call_minutes_minutes_positive CHECK (minutes >= 0),
    CONSTRAINT pk_call_minutes PRIMARY KEY (account_id, utc_day)
) STRICT;

-- The month total across every account (global free capacity).
CREATE INDEX index_call_minutes_on_utc_day ON call_minutes (utc_day);
