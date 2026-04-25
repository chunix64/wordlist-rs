use std::fs;

use rtf_parser::{Lexer, Parser};

use crate::parser::processor::WordListParser;

pub struct RtfParser;

impl WordListParser for RtfParser {
    fn extract_all_text(&self, file: &std::path::Path) -> Option<String> {
        if let Ok(rtf_input) = fs::read_to_string(file)
            && let Ok(rtf_token) = Lexer::scan(&rtf_input)
            && let Ok(doc) = Parser::new(rtf_token).parse()
        {
            return Some(doc.get_text());
        };

        None
    }
}
