CREATE TABLE IF NOT EXISTS bug_reports
(
        id UUID NOT NULL DEFAULT uuid_generate_v4(),
        title TEXT NOT NULL,
        organization_affected TEXT NOT NULL,
        content TEXT NOT NULL,
        wallet_address TEXT NOT NULL,
        feedback TEXT NOT NULL DEFAULT 'NO FEEDBACK YET',
        status SMALLINT NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        CONSTRAINT bug_reports_pkey PRIMARY KEY (id)
)
WITH (OIDS = FALSE);
