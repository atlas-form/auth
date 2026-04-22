#!/usr/bin/env bash
set -euo pipefail

DATABASE_URL="${DATABASE_URL:-${1:-}}"

if [ -z "$DATABASE_URL" ]; then
  echo "Usage: DATABASE_URL=postgres://... $0"
  echo "   or: $0 postgres://..."
  exit 1
fi

echo "Upgrading users/mfa UUID columns on target database..."

psql "$DATABASE_URL" <<'SQL'
BEGIN;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM users
        WHERE id::text !~* '^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$'
    ) THEN
        RAISE EXCEPTION 'users.id contains non-UUID values';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM mfa
        WHERE id::text !~* '^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$'
    ) THEN
        RAISE EXCEPTION 'mfa.id contains non-UUID values';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM mfa
        WHERE user_id::text !~* '^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$'
    ) THEN
        RAISE EXCEPTION 'mfa.user_id contains non-UUID values';
    END IF;
END $$;

ALTER TABLE mfa DROP CONSTRAINT IF EXISTS mfa_user_id_fkey;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'users'
          AND column_name = 'id'
          AND data_type <> 'uuid'
    ) THEN
        ALTER TABLE users
            ALTER COLUMN id TYPE uuid
            USING id::uuid;
    END IF;

    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'mfa'
          AND column_name = 'id'
          AND data_type <> 'uuid'
    ) THEN
        ALTER TABLE mfa
            ALTER COLUMN id TYPE uuid
            USING id::uuid;
    END IF;

    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'mfa'
          AND column_name = 'user_id'
          AND data_type <> 'uuid'
    ) THEN
        ALTER TABLE mfa
            ALTER COLUMN user_id TYPE uuid
            USING user_id::uuid;
    END IF;
END $$;

ALTER TABLE mfa
    ADD CONSTRAINT mfa_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

COMMIT;
SQL

echo "UUID column upgrade completed."
