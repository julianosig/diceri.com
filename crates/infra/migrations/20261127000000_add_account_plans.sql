-- Every account gets a plan, which sets its call limits. Everyone starts on `free`;
-- the others are assigned by hand (`diceri-api plan <username> <plan>`) until they are sold.
-- A new NOT NULL column with a default is safe for a running older release (it never
-- writes the column, so rows it inserts take the default).
ALTER TABLE accounts ADD COLUMN plan TEXT NOT NULL DEFAULT 'free'
    CONSTRAINT check_accounts_plan_enum CHECK (plan IN ('free', 'pro', 'team'));
