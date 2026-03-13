-- Emergency Fix: Move Visit Tables from public to church schema
-- This script safely migrates data from public.* to church.*

BEGIN;

-- 1. Check if church schema exists, create if not
CREATE SCHEMA IF NOT EXISTS church;

-- 2. Create ENUM type in church schema if needed
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type t
                   JOIN pg_namespace n ON t.typnamespace = n.oid
                   WHERE n.nspname = 'church' AND t.typname = 'visit_status') THEN
        CREATE TYPE church.visit_status AS ENUM ('pending', 'in_progress', 'completed', 'cancelled');
    END IF;
END $$;

-- 3. Create visitable_families in church schema if it doesn't exist
CREATE TABLE IF NOT EXISTS church.visitable_families (
    id BIGSERIAL PRIMARY KEY,
    family_name VARCHAR NOT NULL,
    address_street VARCHAR NOT NULL,
    address_city VARCHAR NOT NULL,
    address_postal VARCHAR,
    latitude DECIMAL(10,8),
    longitude DECIMAL(11,8),
    phone VARCHAR,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. Create unique constraint if not exists
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'unique_family_address') THEN
        ALTER TABLE church.visitable_families
        ADD CONSTRAINT unique_family_address UNIQUE (family_name, address_street, address_city);
    END IF;
END $$;

-- 5. Create indexes if not exist
CREATE INDEX IF NOT EXISTS idx_visitable_families_name ON church.visitable_families(family_name);
CREATE INDEX IF NOT EXISTS idx_visitable_families_location ON church.visitable_families(latitude, longitude);

-- 6. Create visit_assignments in church schema if it doesn't exist
CREATE TABLE IF NOT EXISTS church.visit_assignments (
    id BIGSERIAL PRIMARY KEY,
    family_id BIGINT NOT NULL,
    assigned_to_user_id BIGINT NOT NULL,
    scheduled_date DATE NOT NULL,
    status church.visit_status NOT NULL DEFAULT 'pending',
    arrived_at TIMESTAMP,
    arrived_latitude DECIMAL(10,8),
    arrived_longitude DECIMAL(11,8),
    completed_at TIMESTAMP,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_visit_assignments_family_id
        FOREIGN KEY (family_id)
        REFERENCES church.visitable_families(id)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT fk_visit_assignments_user_id
        FOREIGN KEY (assigned_to_user_id)
        REFERENCES church.users(id)
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT chk_arrived_before_completed
        CHECK (arrived_at < completed_at OR completed_at IS NULL)
);

-- 7. Create indexes on visit_assignments if not exist
CREATE INDEX IF NOT EXISTS idx_visit_assignments_user ON church.visit_assignments(assigned_to_user_id);
CREATE INDEX IF NOT EXISTS idx_visit_assignments_family ON church.visit_assignments(family_id);
CREATE INDEX IF NOT EXISTS idx_visit_assignments_date ON church.visit_assignments(scheduled_date);

-- 8. Copy data from public to church if tables exist in public
-- Copy visitable_families
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables
               WHERE table_schema = 'public' AND table_name = 'visitable_families') THEN

        -- Insert data from public to church (skip duplicates based on unique constraint)
        INSERT INTO church.visitable_families
            (id, family_name, address_street, address_city, address_postal,
             latitude, longitude, phone, notes, created_at, updated_at)
        SELECT id, family_name, address_street, address_city, address_postal,
               latitude, longitude, phone, notes, created_at, updated_at
        FROM public.visitable_families
        ON CONFLICT (family_name, address_street, address_city) DO NOTHING;

        -- Update sequence to max id
        PERFORM setval('church.visitable_families_id_seq',
                      (SELECT COALESCE(MAX(id), 1) FROM church.visitable_families));

        RAISE NOTICE 'Copied data from public.visitable_families to church.visitable_families';
    END IF;
END $$;

-- Copy visit_assignments
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables
               WHERE table_schema = 'public' AND table_name = 'visit_assignments') THEN

        -- Insert data from public to church (skip duplicates)
        INSERT INTO church.visit_assignments
            (id, family_id, assigned_to_user_id, scheduled_date, status,
             arrived_at, arrived_latitude, arrived_longitude, completed_at,
             notes, created_at, updated_at)
        SELECT id, family_id, assigned_to_user_id, scheduled_date,
               status::church.visit_status,
               arrived_at, arrived_latitude, arrived_longitude, completed_at,
               notes, created_at, updated_at
        FROM public.visit_assignments
        ON CONFLICT (id) DO NOTHING;

        -- Update sequence to max id
        PERFORM setval('church.visit_assignments_id_seq',
                      (SELECT COALESCE(MAX(id), 1) FROM church.visit_assignments));

        RAISE NOTICE 'Copied data from public.visit_assignments to church.visit_assignments';
    END IF;
END $$;

-- 9. Drop public tables (ONLY after successful copy)
DROP TABLE IF EXISTS public.visit_assignments CASCADE;
DROP TABLE IF EXISTS public.visitable_families CASCADE;
DROP TYPE IF EXISTS public.visit_status CASCADE;

COMMIT;

-- Verification
SELECT 'church.visitable_families' as table_name, COUNT(*) as row_count FROM church.visitable_families
UNION ALL
SELECT 'church.visit_assignments', COUNT(*) FROM church.visit_assignments;
