use std::{collections::HashMap, path::Path};

use epub::doc::EpubDoc;

use crate::utils::text::{clean_word, extract_text_from_html};

pub fn parse_word_list(file: &Path) -> Option<HashMap<String, u64>> {
    println!("[INFO] Processing file: {}", file.display());
    let mut word_list = HashMap::new();

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
        let text = extract_text_from_html(&html);

        for word in text.split_whitespace() {
            let clean = clean_word(word);

            if !clean.is_empty() {
                *word_list.entry(clean).or_insert(0) += 1;
            }
        }

        if !doc.go_next() {
            break;
        }
    }

    Some(word_list)
}
