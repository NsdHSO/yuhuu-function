-- Move seaql_migrations table from public to church schema
-- Run this BEFORE running migrations with the fixed code

BEGIN;

-- 1. Create church schema if it doesn't exist
CREATE SCHEMA IF NOT EXISTS church;

-- 2. If seaql_migrations exists in public, move it to church
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'seaql_migrations'
    ) THEN
        -- Move the table to church schema
        ALTER TABLE public.seaql_migrations SET SCHEMA church;
        RAISE NOTICE 'Moved public.seaql_migrations to church.seaql_migrations';
    ELSE
        RAISE NOTICE 'public.seaql_migrations does not exist, nothing to move';
    END IF;
END $$;

-- 3. Verify the move
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'church' AND table_name = 'seaql_migrations'
    ) THEN
        RAISE NOTICE 'SUCCESS: church.seaql_migrations exists';
    ELSE
        RAISE NOTICE 'WARNING: church.seaql_migrations does not exist yet (will be created on first migration run)';
    END IF;
END $$;

COMMIT;

-- Show current migration status
SELECT version, applied_at FROM church.seaql_migrations ORDER BY version;
