use std::{collections::HashMap, fs, path::Path};

use crate::utils::text::clean_word;

pub fn parse_word_list(file: &Path) -> Option<HashMap<String, u64>> {
    let mut word_list = HashMap::new();
    let all_text = fs::read_to_string(file).unwrap();

    for word in all_text.split_whitespace() {
        let clean = clean_word(word);

        if !clean.is_empty() {
            *word_list.entry(clean).or_insert(0) += 1;
        }
    }

    Some(word_list)
}
