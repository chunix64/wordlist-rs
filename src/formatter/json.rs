use super::{Formatter, sort_word_list};
use crate::cli::SortOrder;
use serde::Serialize;
use std::collections::HashMap;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, word_list: &HashMap<String, u64>, sort_order: &SortOrder) -> String {
        let items = sort_word_list(word_list, sort_order);
        let result: Vec<WordEntry> = items
            .into_iter()
            .map(|(word, frequency)| WordEntry {
                word,
                frequency: *frequency,
            })
            .collect();

        serde_json::to_string(&result).unwrap()
    }
}

#[derive(Serialize)]
struct WordEntry<'a> {
    word: &'a str,
    frequency: u64,
}
