use crate::parser::processor::WordListParser;

pub struct DocxParser;

impl WordListParser for DocxParser {
    fn extract_all_text(&self, file: &std::path::Path) -> Option<String> {
        if let Ok(text) = docx_lite::extract_text(file) {
            return Some(text);
        }

        None
    }
}
