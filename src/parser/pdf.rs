use pdf_oxide::PdfDocument;
use std::path::Path;

use crate::parser::processor::WordListParser;

pub struct PdfParser;

impl WordListParser for PdfParser {
    fn extract_all_text(&self, file: &Path) -> Option<String> {
        if let Ok(mut doc) = PdfDocument::open(file) {
            return doc.extract_all_text().ok();
        }

        None
    }
}
