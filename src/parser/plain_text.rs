use std::{fs, path::Path};

use crate::parser::processor::WordListParser;

pub struct PlainTextParser;

impl WordListParser for PlainTextParser {
    fn extract_all_text(&self, file: &Path) -> Option<String> {
        fs::read_to_string(file).ok()
    }
}
