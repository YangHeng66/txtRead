/*
 * @Author: YangHeng66 yangheng66@gmail.com
 * @Date: 2026-04-21 10:56:23
 * @LastEditors: YangHeng66 yangheng66@gmail.com
 * @LastEditTime: 2026-04-21 10:57:47
 * @FilePath: \txtRead\src-tauri\src\parser\txt.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use crate::error::{AppError, AppResult};
use crate::parser::{ParsedBook, ParsedChapter};
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

pub fn parse(_path: &Path) -> AppResult<ParsedBook> {
    Err(AppError::Parse("not impl".into()))
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
        // "中文" in GBK: D6 D0 CE C4
        let bytes = vec![0xD6, 0xD0, 0xCE, 0xC4];
        assert_eq!(decode_to_utf8(&bytes), "中文");
    }
}
