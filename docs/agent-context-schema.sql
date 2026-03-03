-- agent-context-schema.sql — v1 DDL for mx20022 agent context database
-- 11 tables: schema_meta, iterations, sessions, entries, items,
-- item_dependencies, concepts, item_concepts, patterns, pattern_concepts, knowledge

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Project iterations (milestones / phases)
CREATE TABLE IF NOT EXISTS iterations (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    status      TEXT NOT NULL DEFAULT 'planned'
                CHECK (status IN ('planned', 'active', 'completed', 'blocked')),
    maps_to     TEXT,          -- release version (e.g., 'v0.1')
    description TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Work sessions
CREATE TABLE IF NOT EXISTS sessions (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    ended_at   TEXT,
    summary    TEXT,
    model      TEXT,           -- which model/agent
    branch     TEXT            -- git branch if relevant
);

-- Session log entries (decisions, progress, blockers, discoveries, notes)
CREATE TABLE IF NOT EXISTS entries (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id  INTEGER NOT NULL REFERENCES sessions(id),
    entry_type  TEXT NOT NULL
                CHECK (entry_type IN ('decision', 'progress', 'blocker', 'discovery', 'note')),
    title       TEXT NOT NULL,
    content     TEXT NOT NULL,
    item_id     INTEGER REFERENCES items(id),  -- optional link to a task
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Work items (tasks)
CREATE TABLE IF NOT EXISTS items (
    id           INTEGER PRIMARY KEY,
    iteration_id INTEGER NOT NULL REFERENCES iterations(id),
    title        TEXT NOT NULL,
    description  TEXT,
    status       TEXT NOT NULL DEFAULT 'not_started'
                 CHECK (status IN ('not_started', 'in_progress', 'blocked', 'complete', 'deferred')),
    priority     TEXT NOT NULL DEFAULT 'medium'
                 CHECK (priority IN ('critical', 'high', 'medium', 'low')),
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Task dependency graph
CREATE TABLE IF NOT EXISTS item_dependencies (
    item_id    INTEGER NOT NULL REFERENCES items(id),
    depends_on INTEGER NOT NULL REFERENCES items(id),
    PRIMARY KEY (item_id, depends_on),
    CHECK (item_id != depends_on)
);

-- Domain concepts (tags for cross-referencing)
CREATE TABLE IF NOT EXISTS concepts (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

-- Many-to-many: items ↔ concepts
CREATE TABLE IF NOT EXISTS item_concepts (
    item_id    INTEGER NOT NULL REFERENCES items(id),
    concept_id INTEGER NOT NULL REFERENCES concepts(id),
    PRIMARY KEY (item_id, concept_id)
);

-- Reusable patterns and conventions
CREATE TABLE IF NOT EXISTS patterns (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    example     TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Many-to-many: patterns ↔ concepts
CREATE TABLE IF NOT EXISTS pattern_concepts (
    pattern_id INTEGER NOT NULL REFERENCES patterns(id),
    concept_id INTEGER NOT NULL REFERENCES concepts(id),
    PRIMARY KEY (pattern_id, concept_id)
);

-- Queryable knowledge base (protocol + domain + config + reference)
CREATE TABLE IF NOT EXISTS knowledge (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT NOT NULL,    -- 'protocol', 'architecture', 'api', 'reference', 'config', 'testing', 'domain'
    title    TEXT NOT NULL,
    content  TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(category, title)
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_items_iteration ON items(iteration_id);
CREATE INDEX IF NOT EXISTS idx_items_status ON items(status);
CREATE INDEX IF NOT EXISTS idx_entries_session ON entries(session_id);
CREATE INDEX IF NOT EXISTS idx_entries_type ON entries(entry_type);
CREATE INDEX IF NOT EXISTS idx_knowledge_category ON knowledge(category);
CREATE INDEX IF NOT EXISTS idx_item_concepts_concept ON item_concepts(concept_id);
CREATE INDEX IF NOT EXISTS idx_item_dependencies_dep ON item_dependencies(depends_on);
