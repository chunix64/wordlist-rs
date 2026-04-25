use epub::doc::EpubDoc;
use std::path::Path;

use crate::{parser::WordListParser, utils::text::extract_text_from_html};

pub struct EpubParser;

impl WordListParser for EpubParser {
    fn extract_all_text(&self, file: &Path) -> Option<String> {
        let mut all_text = String::new();

        let mut doc = match EpubDoc::new(file) {
            Ok(doc) => doc,
            Err(error) => {
                eprintln!("[ERROR] Failed to parse file: {}", file.display());
                eprintln!("[ERROR] Error: {:?}", error);
                return None;
            }
        };

        while let Some((content, _)) = doc.get_current() {
            let html = String::from_utf8_lossy(&content);
            all_text += &extract_text_from_html(&html);

            if !doc.go_next() {
                break;
            }
        }

        Some(all_text)
    }
}
