use crate::error::AppResult;
use rusqlite::{params, Connection, OptionalExtension};

pub fn set(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub fn get(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    Ok(stmt
        .query_row(params![key], |r| r.get::<_, String>(0))
        .optional()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../../migrations/001_init.sql")).unwrap();
        c
    }

    #[test]
    fn set_and_get() {
        let c = mem();
        set(&c, "theme", "dark").unwrap();
        assert_eq!(get(&c, "theme").unwrap(), Some("dark".into()));
        set(&c, "theme", "light").unwrap();
        assert_eq!(get(&c, "theme").unwrap(), Some("light".into()));
    }
}
