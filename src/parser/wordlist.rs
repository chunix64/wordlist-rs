use std::collections::HashMap;

use crate::utils::text::clean_word;

pub fn generate_word_list(text: &str) -> Option<HashMap<String, u64>> {
    let mut word_list = HashMap::new();

    for word in text.split_whitespace() {
        let clean = clean_word(word);

        if !clean.is_empty() {
            *word_list.entry(clean).or_insert(0) += 1;
        }
    }

    Some(word_list)
}

pub fn merge_word_list(word_list_1: &mut HashMap<String, u64>, word_list_2: HashMap<String, u64>) {
    for (word, frequency) in word_list_2 {
        *word_list_1.entry(word).or_insert(0) += frequency;
    }
}

pub fn get_word_list_size(word_list: &HashMap<String, u64>) -> (u64, u64) {
    let mut word_list_size = (0, 0);

    for frequency in word_list.values() {
        word_list_size.0 += 1;
        word_list_size.1 += *frequency;
    }

    word_list_size
}
