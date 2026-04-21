use crate::error::AppResult;
use crate::models::Book;
use rusqlite::{params, Connection};

pub fn insert(
    conn: &Connection,
    title: &str,
    author: Option<&str>,
    format: &str,
    source_path: &str,
    cover: Option<&[u8]>,
) -> AppResult<i64> {
    conn.execute(
        "INSERT INTO books (title, author, format, source_path, cover_blob) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![title, author, format, source_path, cover],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list(conn: &Connection) -> AppResult<Vec<Book>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, author, format, source_path, created_at FROM books ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Book {
            id: r.get(0)?,
            title: r.get(1)?,
            author: r.get(2)?,
            format: r.get(3)?,
            source_path: r.get(4)?,
            created_at: r.get(5)?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
    conn.execute("DELETE FROM books WHERE id = ?1", params![id])?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_cover(conn: &Connection, id: i64) -> AppResult<Option<Vec<u8>>> {
    let mut stmt = conn.prepare("SELECT cover_blob FROM books WHERE id = ?1")?;
    let blob: Option<Vec<u8>> = stmt.query_row(params![id], |r| r.get(0)).ok().flatten();
    Ok(blob)
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
    fn insert_and_list() {
        let c = mem();
        let id = insert(&c, "t1", Some("a1"), "txt", "/x", None).unwrap();
        assert!(id > 0);
        let v = list(&c).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].title, "t1");
    }

    #[test]
    fn delete_removes() {
        let c = mem();
        let id = insert(&c, "t", None, "txt", "/p", None).unwrap();
        delete(&c, id).unwrap();
        assert_eq!(list(&c).unwrap().len(), 0);
    }
}
