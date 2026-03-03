#!/usr/bin/env bash
# migrate-context-v1.sh — Idempotent migration for agent context database v1
set -euo pipefail

DB_PATH="${1:-./docs/agent_context.db}"
SCHEMA_PATH="$(dirname "$0")/agent-context-schema.sql"
SEED_PATH="$(dirname "$0")/seed-context.sql"

if [ ! -f "$SCHEMA_PATH" ]; then
    echo "ERROR: Schema file not found at $SCHEMA_PATH" >&2
    exit 1
fi

# Check if DB exists and has a version
if [ -f "$DB_PATH" ]; then
    CURRENT_VERSION=$(sqlite3 "$DB_PATH" "SELECT value FROM schema_meta WHERE key='version';" 2>/dev/null || echo "")
    if [ "$CURRENT_VERSION" = "1" ]; then
        echo "Database already at v1, skipping migration."
        exit 0
    fi
    echo "Upgrading existing database to v1..."
else
    echo "Creating new database at $DB_PATH..."
fi

# Apply schema (IF NOT EXISTS makes this safe for partial migrations)
sqlite3 "$DB_PATH" < "$SCHEMA_PATH"

# Set version
sqlite3 "$DB_PATH" "INSERT OR REPLACE INTO schema_meta (key, value) VALUES ('version', '1');"

echo "Schema v1 applied successfully."

# Apply seed data if it exists and knowledge table is empty
if [ -f "$SEED_PATH" ]; then
    ROW_COUNT=$(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM knowledge;" 2>/dev/null || echo "0")
    if [ "$ROW_COUNT" = "0" ]; then
        echo "Seeding database..."
        sqlite3 "$DB_PATH" < "$SEED_PATH"
        echo "Seed data applied."
    else
        echo "Database already has data, skipping seed."
    fi
fi

echo "Migration complete. DB at $DB_PATH"
