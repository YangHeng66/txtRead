use crate::error::AppResult;
use crate::models::Progress;
use rusqlite::{params, Connection, OptionalExtension};

pub fn upsert(conn: &Connection, p: &Progress) -> AppResult<()> {
    conn.execute(
        "INSERT INTO progress (book_id, chapter_idx, scroll_ratio, updated_at)
         VALUES (?1, ?2, ?3, strftime('%s','now'))
         ON CONFLICT(book_id) DO UPDATE SET
            chapter_idx = excluded.chapter_idx,
            scroll_ratio = excluded.scroll_ratio,
            updated_at = excluded.updated_at",
        params![p.book_id, p.chapter_idx, p.scroll_ratio],
    )?;
    Ok(())
}

pub fn get(conn: &Connection, book_id: i64) -> AppResult<Option<Progress>> {
    let mut stmt = conn.prepare(
        "SELECT book_id, chapter_idx, scroll_ratio FROM progress WHERE book_id = ?1",
    )?;
    let p = stmt
        .query_row(params![book_id], |r| {
            Ok(Progress {
                book_id: r.get(0)?,
                chapter_idx: r.get(1)?,
                scroll_ratio: r.get(2)?,
            })
        })
        .optional()?;
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::books;

    fn mem() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../../migrations/001_init.sql")).unwrap();
        c
    }

    #[test]
    fn upsert_and_get() {
        let c = mem();
        let bid = books::insert(&c, "b", None, "txt", "/p", None).unwrap();
        upsert(&c, &Progress { book_id: bid, chapter_idx: 3, scroll_ratio: 0.42 }).unwrap();
        let got = get(&c, bid).unwrap().unwrap();
        assert_eq!(got.chapter_idx, 3);
        assert!((got.scroll_ratio - 0.42).abs() < 1e-6);

        upsert(&c, &Progress { book_id: bid, chapter_idx: 5, scroll_ratio: 0.9 }).unwrap();
        let got = get(&c, bid).unwrap().unwrap();
        assert_eq!(got.chapter_idx, 5);
    }
}
