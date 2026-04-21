use crate::error::AppResult;
use crate::models::SearchHit;
use rusqlite::{params, Connection};

pub fn query(
    conn: &Connection,
    book_id: Option<i64>,
    q: &str,
    limit: i64,
) -> AppResult<Vec<SearchHit>> {
    // Wrap the term in double quotes so FTS5 treats it as a phrase and
    // punctuation/operators in user input don't misbehave.
    let safe = q.replace('"', "\"\"");
    let fts_query = format!("\"{}\"", safe);

    let sql = if book_id.is_some() {
        "SELECT b.id, b.title, c.idx, c.title,
                snippet(chapters_fts, 1, '<mark>', '</mark>', '…', 10) AS snip
         FROM chapters_fts f
         JOIN chapters c ON c.id = f.rowid
         JOIN books b ON b.id = c.book_id
         WHERE chapters_fts MATCH ?1 AND b.id = ?2
         LIMIT ?3"
    } else {
        "SELECT b.id, b.title, c.idx, c.title,
                snippet(chapters_fts, 1, '<mark>', '</mark>', '…', 10) AS snip
         FROM chapters_fts f
         JOIN chapters c ON c.id = f.rowid
         JOIN books b ON b.id = c.book_id
         WHERE chapters_fts MATCH ?1
         LIMIT ?2"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows: Vec<SearchHit> = if let Some(bid) = book_id {
        stmt.query_map(params![fts_query, bid, limit], map_hit)?
            .filter_map(Result::ok)
            .collect()
    } else {
        stmt.query_map(params![fts_query, limit], map_hit)?
            .filter_map(Result::ok)
            .collect()
    };
    Ok(rows)
}

fn map_hit(r: &rusqlite::Row) -> rusqlite::Result<SearchHit> {
    Ok(SearchHit {
        book_id: r.get(0)?,
        book_title: r.get(1)?,
        chapter_idx: r.get(2)?,
        chapter_title: r.get(3)?,
        snippet: r.get(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{books, chapters};

    fn mem() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../../migrations/001_init.sql")).unwrap();
        c
    }

    #[test]
    fn find_word() {
        let mut c = mem();
        let bid = books::insert(&c, "b", None, "txt", "/p", None).unwrap();
        chapters::insert_many(
            &mut c,
            bid,
            &[
                ("c1".into(), "quick brown fox".into()),
                ("c2".into(), "lazy dog sleeps".into()),
            ],
        )
        .unwrap();

        let hits = query(&c, None, "brown", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].chapter_idx, 0);
        assert!(hits[0].snippet.contains("brown"));
    }
}
