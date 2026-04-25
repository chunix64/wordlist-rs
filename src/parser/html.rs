use std::fs;

use crate::{parser::processor::WordListParser, utils::text::extract_text_from_html};

pub struct HtmlParser;

impl WordListParser for HtmlParser {
    fn extract_all_text(&self, file: &std::path::Path) -> Option<String> {
        if let Ok(html) = fs::read_to_string(file) {
            return Some(extract_text_from_html(&html));
        };

        None
    }
}
