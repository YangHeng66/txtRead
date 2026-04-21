use crate::error::{AppError, AppResult};
use crate::parser::{ParsedBook, ParsedChapter};
use epub::doc::{EpubDoc, NavPoint};
use std::collections::HashMap;
use std::path::Path;

pub fn parse(path: &Path) -> AppResult<ParsedBook> {
    let mut doc = EpubDoc::new(path).map_err(|e| AppError::Epub(e.to_string()))?;

    let title = doc
        .mdata("title")
        .map(|m| m.value.clone())
        .unwrap_or_else(|| "untitled".into());
    let author = doc.mdata("creator").map(|m| m.value.clone());
    let cover = doc.get_cover().map(|(bytes, _mime)| bytes);

    // Build resource path (normalized) -> TOC label. First label wins per path,
    // fragments (#id) stripped. Handles EPUBs where TOC splits one HTML file by anchor.
    let toc = doc.toc.clone();
    let mut toc_labels: HashMap<String, String> = HashMap::new();
    for np in &toc {
        collect_toc_labels(np, &mut toc_labels);
    }

    let spine_len = doc.spine.len();
    let mut chapters: Vec<ParsedChapter> = Vec::with_capacity(spine_len);

    for i in 0..spine_len {
        if !doc.set_current_chapter(i) {
            continue;
        }
        let idref = doc.spine[i].idref.clone();
        let res_path = doc
            .resources
            .get(&idref)
            .map(|r| normalize_path(&r.path.to_string_lossy()))
            .unwrap_or_default();

        let Some((html, _mime)) = doc.get_current_str() else {
            continue;
        };
        let text = html_to_text(&html);
        if text.trim().is_empty() {
            continue;
        }

        let title = toc_labels
            .get(&res_path)
            .cloned()
            .unwrap_or_else(|| format!("Chapter {}", chapters.len() + 1));

        chapters.push(ParsedChapter {
            title,
            content: text,
        });
    }

    if chapters.is_empty() {
        return Err(AppError::Epub("no readable chapters".into()));
    }

    Ok(ParsedBook {
        title,
        author,
        format: "epub",
        chapters,
        cover,
    })
}

fn collect_toc_labels(np: &NavPoint, out: &mut HashMap<String, String>) {
    let raw = np.content.to_string_lossy().to_string();
    let clean = raw.split('#').next().unwrap_or(&raw).to_string();
    let key = normalize_path(&clean);
    out.entry(key).or_insert_with(|| np.label.clone());
    for child in &np.children {
        collect_toc_labels(child, out);
    }
}

fn normalize_path(p: &str) -> String {
    p.replace('\\', "/")
}

/// Strip HTML tags to plain text while preserving paragraph breaks.
fn html_to_text(html: &str) -> String {
    use once_cell::sync::Lazy;
    use regex::Regex;
    static RE_BLOCK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?i)</?(p|div|br\s*/?|h[1-6]|li)\b[^>]*>").unwrap());
    static RE_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());
    static RE_WS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \t]+").unwrap());
    static RE_NL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").unwrap());

    let s = RE_BLOCK.replace_all(html, "\n\n");
    let s = RE_TAG.replace_all(&s, "");
    let s = html_escape_decode(&s);
    let s = RE_WS.replace_all(&s, " ");
    let s = RE_NL.replace_all(&s, "\n\n");
    s.trim().to_string()
}

fn html_escape_decode(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/alice.epub")
    }

    #[test]
    fn parses_alice() {
        let p = fixture();
        if !p.exists() {
            return;
        }
        let book = parse(&p).expect("parse");
        assert!(!book.title.is_empty());
        assert!(book.chapters.len() >= 2);
        let joined: String = book.chapters.iter().map(|c| c.content.as_str()).collect();
        assert!(joined.to_lowercase().contains("alice"));
    }
}
