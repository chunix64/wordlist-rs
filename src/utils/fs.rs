use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    cli::{OutputFormat, SortOrder},
    formatter::get_formatter,
};

pub fn collect_files(paths: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            if recursive {
                for entry in walkdir::WalkDir::new(path) {
                    let entry = entry.unwrap();
                    if entry.path().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
            } else {
                for entry in std::fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    if entry.path().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    files
}

pub fn save_word_list(
    word_list: &HashMap<String, u64>,
    output: &Option<PathBuf>,
    output_format: &OutputFormat,
    sort_order: &SortOrder,
) {
    let formatter = get_formatter(output_format);
    let content = formatter.format(word_list, sort_order);

    let file_name: String = match output {
        Some(path) => path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("output")
            .to_string(),
        None => "output".to_string(),
    };

    let path = format!("{}.{}", file_name, output_format.as_str());

    fs::write(path, content).unwrap();
}
