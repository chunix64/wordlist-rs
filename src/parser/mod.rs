pub mod epub;
pub mod pdf;
pub mod plain_text;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    parser::{epub::EpubParser, pdf::PdfParser, plain_text::PlainTextParser},
    utils::text::clean_word,
};

pub trait WordListParser {
    fn extract_all_text(&self, file: &Path) -> Option<String>;
}

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

fn merge_word_list(word_list_1: &mut HashMap<String, u64>, word_list_2: HashMap<String, u64>) {
    for (word, frequency) in word_list_2 {
        *word_list_1.entry(word).or_insert(0) += frequency;
    }
}

fn get_parser(file: &Path) -> Option<Box<dyn WordListParser>> {
    if let Some(ext) = file.extension().and_then(|ext| ext.to_str()) {
        match ext {
            "txt" => return Some(Box::new(PlainTextParser)),
            "md" => return Some(Box::new(PlainTextParser)),
            "log" => return Some(Box::new(PlainTextParser)),
            _ => {}
        }
    }

    match infer::get_from_path(file) {
        Ok(Some(kind)) if kind.mime_type() == "application/epub+zip" => {
            return Some(Box::new(EpubParser));
        }
        Ok(Some(kind)) if kind.mime_type() == "application/pdf" => {
            return Some(Box::new(PdfParser));
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

fn parse_file(file: &Path) -> Option<String> {
    println!("[INFO] Processing file: {}", file.display());
    let parser = get_parser(file)?;
    parser.extract_all_text(file)
}

pub fn parse_word_list(files: Vec<PathBuf>) -> HashMap<String, u64> {
    let mut word_list = HashMap::new();

    for file in files {
        if let Some(text) = parse_file(&file)
            && let Some(partial) = generate_word_list(&text)
        {
            merge_word_list(&mut word_list, partial);
        }
    }

    word_list
}
