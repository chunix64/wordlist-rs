pub mod epub;
pub mod pdf;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn merge_word_list(word_list_1: &mut HashMap<String, u64>, word_list_2: HashMap<String, u64>) {
    for (word, frequency) in word_list_2 {
        *word_list_1.entry(word).or_insert(0) += frequency;
    }
}

fn parse_file(file: &Path) -> Option<HashMap<String, u64>> {
    match infer::get_from_path(file) {
        Ok(Some(kind)) if kind.mime_type() == "application/epub+zip" => {
            return epub::parse_word_list(file)
        }
        Ok(Some(kind)) => {
            println!("[INFO] Skipping unsupported type: {}", kind.mime_type());
        }
        Ok(None) => {
            println!("[WARN] Unknown file type: {:?}", file);
        }
        Err(error) => {
            eprintln!("[ERROR] Failed to read file {:?}: {}", file, error);
        }
    }

    None
}

pub fn parse_word_list(files: Vec<PathBuf>) -> HashMap<String, u64> {
    let mut word_list = HashMap::new();

    for file in files {
        if let Some(partial) = parse_file(&file) {
            merge_word_list(&mut word_list, partial);
        }
    }

    word_list
}
