use super::{Formatter, sort_word_list};
use crate::cli::SortOrder;
use std::collections::HashMap;
use std::fmt::Write;

pub struct TxtFormatter;

impl Formatter for TxtFormatter {
    fn format(&self, word_list: &HashMap<String, u64>, sort_order: &SortOrder) -> String {
        let items = sort_word_list(word_list, sort_order);
        let mut result = String::new();

        for (i, (key, value)) in items.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }

            write!(result, "{key} {value}").unwrap();
        }

        result
    }
}
