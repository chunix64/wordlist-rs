use crate::parser::WordListParser;
use std::{fs, path::Path};

pub struct PlainTextParser;

impl WordListParser for PlainTextParser {
    fn extract_all_text(&self, file: &Path) -> Option<String> {
        fs::read_to_string(file).ok()
    }
}
