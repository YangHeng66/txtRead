use crate::error::AppResult;
use crate::parser::{ParsedBook, ParsedChapter};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

pub fn decode_to_utf8(bytes: &[u8]) -> String {
    // strip UTF-8 BOM
    let body = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };

    // try UTF-8 first (fast path, no allocation when valid)
    if let Ok(s) = std::str::from_utf8(body) {
        return s.to_string();
    }

    // fall back to chardetng
    let mut det = chardetng::EncodingDetector::new();
    det.feed(body, true);
    let enc = det.guess(None, true);
    let (cow, _, _) = enc.decode(body);
    cow.into_owned()
}

// Chapter regex — plan used `\`-continued raw string (invalid in Rust raw strings,
// yields `\<newline>` which regex rejects). Using `concat!` of raw literals instead.
// Simple markers (楔子/前言/尾声/终章/后记/序X) must stand alone on their line, so
// they have no `.{0,40}` tail — this prevents `前言正文` being matched as a chapter
// title and lets it fall through to the prefix-as-"前言" branch.
static CHAPTER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        r"(?m)^\s*(?:",
        r"第[一二三四五六七八九十百千万零〇两\d]+[章回节卷篇集部幕折].{0,40}",
        r"|[Cc]hapter\s+\d+.{0,40}",
        r"|序[章言幕曲]",
        r"|楔\s*子",
        r"|前\s*言",
        r"|尾\s*声",
        r"|终\s*章",
        r"|后\s*记",
        r")\s*$"
    ))
    .expect("chapter regex")
});

pub fn split_chapters(text: &str) -> Vec<ParsedChapter> {
    let mut out: Vec<ParsedChapter> = Vec::new();

    let marks: Vec<(usize, usize, String)> = CHAPTER_RE
        .find_iter(text)
        .map(|m| (m.start(), m.end(), m.as_str().trim().to_string()))
        .collect();

    if marks.is_empty() {
        return vec![ParsedChapter {
            title: "正文".into(),
            content: text.trim().to_string(),
        }];
    }

    // content before the first title becomes "前言" if non-empty
    if marks[0].0 > 0 {
        let prefix = text[..marks[0].0].trim();
        if !prefix.is_empty() {
            out.push(ParsedChapter {
                title: "前言".into(),
                content: prefix.to_string(),
            });
        }
    }

    for i in 0..marks.len() {
        let (_ts, te, ref title) = marks[i];
        let body_end = if i + 1 < marks.len() { marks[i + 1].0 } else { text.len() };
        let body = text[te..body_end].trim();
        out.push(ParsedChapter {
            title: title.clone(),
            content: body.to_string(),
        });
    }

    out
}

pub fn parse(path: &Path) -> AppResult<ParsedBook> {
    let bytes = std::fs::read(path)?;
    let text = decode_to_utf8(&bytes);
    let chapters = split_chapters(&text);
    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled")
        .to_string();
    Ok(ParsedBook {
        title,
        author: None,
        format: "txt",
        chapters,
        cover: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_utf8_bom() {
        let bytes: Vec<u8> = vec![0xEF, 0xBB, 0xBF, b'h', b'i'];
        assert_eq!(decode_to_utf8(&bytes), "hi");
    }

    #[test]
    fn decodes_utf8_plain() {
        let s = "中文内容";
        assert_eq!(decode_to_utf8(s.as_bytes()), s);
    }

    #[test]
    fn decodes_gbk() {
        let bytes = vec![0xD6, 0xD0, 0xCE, 0xC4];
        assert_eq!(decode_to_utf8(&bytes), "中文");
    }

    #[test]
    fn splits_chinese_chapters() {
        let text = "前言正文\n\
                    第一章 开始\n\
                    内容A\n\
                    第二章 继续\n\
                    内容B\n\
                    第三章 结束\n\
                    内容C";
        let chapters = super::split_chapters(text);
        assert_eq!(chapters.len(), 4);
        assert_eq!(chapters[0].title, "前言");
        assert_eq!(chapters[1].title, "第一章 开始");
        assert!(chapters[1].content.contains("内容A"));
        assert_eq!(chapters[2].title, "第二章 继续");
        assert_eq!(chapters[3].title, "第三章 结束");
    }

    #[test]
    fn splits_english_chapters() {
        let text = "Chapter 1 Dawn\nhello\nChapter 2 Dusk\nworld";
        let chapters = super::split_chapters(text);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].title, "Chapter 1 Dawn");
        assert_eq!(chapters[1].title, "Chapter 2 Dusk");
    }

    #[test]
    fn handles_preface_and_epilogue() {
        let text = "楔子\n起源\n第一章 启程\n路上\n尾声\n终了";
        let chapters = super::split_chapters(text);
        assert_eq!(chapters.len(), 3);
        assert_eq!(chapters[0].title, "楔子");
        assert_eq!(chapters[1].title, "第一章 启程");
        assert_eq!(chapters[2].title, "尾声");
    }

    #[test]
    fn no_chapters_returns_single() {
        let text = "just plain text no chapter markers";
        let chapters = super::split_chapters(text);
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].title, "正文");
    }
}
