use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init()?;
        Ok(db)
    }

    pub fn new_in_memory() -> Result<Self, DatabaseError> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init()?;
        Ok(db)
    }

    pub fn init(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS subscriptions (
                channel_id   TEXT PRIMARY KEY,
                channel_name TEXT NOT NULL,
                avatar_url   TEXT,
                subscribed_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_checked  TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS watch_history (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                video_id      TEXT NOT NULL,
                title         TEXT NOT NULL,
                channel_name  TEXT NOT NULL,
                thumbnail_url TEXT,
                duration_secs INTEGER NOT NULL,
                progress_secs INTEGER NOT NULL DEFAULT 0,
                watched_at    TEXT NOT NULL DEFAULT (datetime('now')),
                UNIQUE(video_id, watched_at)
            );
            CREATE INDEX IF NOT EXISTS idx_history_video ON watch_history(video_id);
            CREATE INDEX IF NOT EXISTS idx_history_date  ON watch_history(watched_at);

            CREATE TABLE IF NOT EXISTS bookmarks (
                video_id      TEXT PRIMARY KEY,
                title         TEXT NOT NULL,
                channel_name  TEXT NOT NULL,
                thumbnail_url TEXT,
                duration_secs INTEGER,
                added_at      TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS settings (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn add_subscription(&self, id: &str, name: &str, avatar: Option<&str>) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO subscriptions (channel_id, channel_name, avatar_url) VALUES (?1, ?2, ?3)",
            params![id, name, avatar],
        )?;
        Ok(())
    }

    pub fn remove_subscription(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM subscriptions WHERE channel_id = ?1", params![id])?;
        Ok(())
    }

    pub fn is_subscribed(&self, id: &str) -> Result<bool, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subscriptions WHERE channel_id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn get_subscriptions(&self) -> Result<Vec<(String, String, Option<String>)>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT channel_id, channel_name, avatar_url FROM subscriptions ORDER BY channel_name ASC")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut subs = Vec::new();
        for s in rows {
            subs.push(s?);
        }
        Ok(subs)
    }
}
