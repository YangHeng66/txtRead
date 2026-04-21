CREATE TABLE IF NOT EXISTS books (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    author      TEXT,
    format      TEXT NOT NULL CHECK(format IN ('txt','epub')),
    source_path TEXT NOT NULL,
    cover_blob  BLOB,
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE TABLE IF NOT EXISTS chapters (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id    INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    idx        INTEGER NOT NULL,
    title      TEXT NOT NULL,
    content    TEXT NOT NULL,
    UNIQUE(book_id, idx)
);
CREATE INDEX IF NOT EXISTS idx_chapters_book ON chapters(book_id);

CREATE TABLE IF NOT EXISTS progress (
    book_id       INTEGER PRIMARY KEY REFERENCES books(id) ON DELETE CASCADE,
    chapter_idx   INTEGER NOT NULL DEFAULT 0,
    scroll_ratio  REAL NOT NULL DEFAULT 0.0,
    updated_at    INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS chapters_fts USING fts5(
    title, content,
    content='chapters', content_rowid='id',
    tokenize='unicode61'
);

CREATE TRIGGER IF NOT EXISTS chapters_ai AFTER INSERT ON chapters BEGIN
    INSERT INTO chapters_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
END;
CREATE TRIGGER IF NOT EXISTS chapters_ad AFTER DELETE ON chapters BEGIN
    INSERT INTO chapters_fts(chapters_fts, rowid, title, content) VALUES('delete', old.id, old.title, old.content);
END;
