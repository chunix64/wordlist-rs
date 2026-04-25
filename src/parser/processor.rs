use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::parser::{
    docx::DocxParser,
    epub::EpubParser,
    html::HtmlParser,
    pdf::PdfParser,
    plain_text::PlainTextParser,
    wordlist::{generate_word_list, merge_word_list},
};

pub trait WordListParser {
    fn extract_all_text(&self, file: &Path) -> Option<String>;
}

fn get_parser(file: &Path) -> Option<Box<dyn WordListParser>> {
    if let Some(ext) = file.extension().and_then(|ext| ext.to_str()) {
        match ext.to_lowercase().as_str() {
            "txt" | "md" | "log" | "rst" | "text" | "csv" | "tsv" => {
                return Some(Box::new(PlainTextParser));
            }
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
        Ok(Some(kind))
            if kind.mime_type()
                == "application/vnd.openxmlformats-officedocument.wordprocessingml.document" =>
        {
            return Some(Box::new(DocxParser));
        }
        Ok(Some(kind)) if kind.mime_type() == "text/html" => {
            return Some(Box::new(HtmlParser));
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
