# Schema Migration Fix - seaql_migrations in Wrong Schema

## What Happened

The `seaql_migrations` tracking table was created in `public` schema instead of `church` schema because:

1. **SeaORM CLI doesn't have a `--database-schema` flag** - the `run_migration.sh` script was passing this flag, but it was being **silently ignored**
2. **No schema override in Migrator** - the default behavior creates `seaql_migrations` in `public` schema

## What Was Fixed

### 1. migration/src/lib.rs
Added `migration_table_name()` override to force seaql_migrations into church schema:

```rust
fn migration_table_name() -> sea_orm::DynIden {
    sea_orm::Alias::new("church.seaql_migrations").into_iden()
}
```

### 2. run_migration.sh
Removed the non-existent `--database-schema` flag:

```bash
# Before (WRONG - flag doesn't exist):
cargo run --manifest-path migration/Cargo.toml -- --database-url "$MODIFIED_URL" --database-schema "$DB_SCHEMA"

# After (CORRECT):
cargo run --manifest-path migration/Cargo.toml -- --database-url "$MODIFIED_URL"
```

## How to Fix Your Database

### Step 1: Move seaql_migrations Table to Church Schema

Run this SQL to move the existing tracking table:

```bash
psql $DATABASE_URL -f move_seaql_migrations_to_church.sql
```

This will:
- Move `public.seaql_migrations` → `church.seaql_migrations`
- Preserve all migration history
- Verify the move was successful

### Step 2: Verify Current State

Check which migrations are recorded:

```sql
-- Should show all your applied migrations
SELECT version, applied_at FROM church.seaql_migrations ORDER BY version;

-- Should be empty now
\dt public.seaql*
```

### Step 3: Future Migrations Will Work Correctly

Now when you run migrations, they will:
- ✅ Use `church.seaql_migrations` for tracking
- ✅ Create all tables in `church` schema (as already defined in migrations)
- ✅ No more tables in `public` schema

## Verification

After applying the fix, verify:

```sql
-- seaql_migrations should be in church schema
\dt church.seaql_migrations

-- All visit tables should be in church schema
\dt church.*visit*

-- Nothing in public (except default postgres tables)
\dt public.*
```

## Why This Matters

- **Data Integrity**: All church application data should be in the `church` schema
- **Permissions**: Easier to manage permissions at schema level
- **Organization**: Clear separation between application data and postgres internals
- **Migrations**: Migration tracking should be with the application data

## Going Forward

The code is now fixed. Future migrations will automatically:
1. Use `church.seaql_migrations` for tracking
2. Create all tables in `church` schema (as specified in each migration with `Alias::new("church")`)

No manual intervention needed for future migrations.
