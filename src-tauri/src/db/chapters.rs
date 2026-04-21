use crate::error::AppResult;
use crate::models::{Chapter, ChapterMeta};
use rusqlite::{params, Connection};

pub fn insert_many(
    conn: &mut Connection,
    book_id: i64,
    chapters: &[(String, String)],
) -> AppResult<()> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO chapters (book_id, idx, title, content) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for (i, (title, content)) in chapters.iter().enumerate() {
            stmt.execute(params![book_id, i as i64, title, content])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn list_meta(conn: &Connection, book_id: i64) -> AppResult<Vec<ChapterMeta>> {
    let mut stmt =
        conn.prepare("SELECT idx, title FROM chapters WHERE book_id = ?1 ORDER BY idx")?;
    let rows = stmt.query_map(params![book_id], |r| {
        Ok(ChapterMeta { idx: r.get(0)?, title: r.get(1)? })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn get_one(conn: &Connection, book_id: i64, idx: i64) -> AppResult<Chapter> {
    let mut stmt = conn.prepare(
        "SELECT idx, title, content FROM chapters WHERE book_id = ?1 AND idx = ?2",
    )?;
    Ok(stmt.query_row(params![book_id, idx], |r| {
        Ok(Chapter { idx: r.get(0)?, title: r.get(1)?, content: r.get(2)? })
    })?)
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
    fn insert_many_then_list() {
        let mut c = mem();
        let bid = books::insert(&c, "b", None, "txt", "/p", None).unwrap();
        insert_many(&mut c, bid, &[("c1".into(), "x".into()), ("c2".into(), "y".into())]).unwrap();
        let metas = list_meta(&c, bid).unwrap();
        assert_eq!(metas.len(), 2);
        assert_eq!(metas[0].title, "c1");
        let ch = get_one(&c, bid, 1).unwrap();
        assert_eq!(ch.title, "c2");
        assert_eq!(ch.content, "y");
    }
}
