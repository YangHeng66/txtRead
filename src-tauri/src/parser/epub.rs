use crate::error::{AppError, AppResult};
use crate::parser::ParsedBook;
use std::path::Path;

pub fn parse(_path: &Path) -> AppResult<ParsedBook> {
    Err(AppError::Epub("not impl".into()))
}
