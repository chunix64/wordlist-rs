use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    cli::{OutputFormat, SortOrder},
    formatter::get_formatter,
};

// Collect files from mixed directories and files
pub fn collect_files(paths: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            if recursive {
                for entry in walkdir::WalkDir::new(path) {
                    match entry {
                        Ok(entry) => {
                            if entry.path().is_file() {
                                files.push(entry.path().to_path_buf());
                            }
                        }
                        Err(error) => {
                            eprintln!("[ERROR] WalkDir error at {}: {}", path.display(), error);
                        }
                    };
                }
            } else {
                match std::fs::read_dir(path) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(entry) => {
                                    if entry.path().is_file() {
                                        files.push(entry.path().to_path_buf());
                                    }
                                }
                                Err(error) => {
                                    eprintln!(
                                        "[ERROR] Failed to read entry in {}: {}",
                                        path.display(),
                                        error
                                    )
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!(
                            "[ERROR] Failed to read directory {}: {}",
                            path.display(),
                            error
                        );
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
    let content = match formatter.format(word_list, sort_order) {
        Some(content) => content,
        None => {
            eprintln!("[ERROR] Failed to format word list");
            return;
        }
    };

    let file_name: String = match output {
        Some(path) => path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("output")
            .to_string(),
        None => "output".to_string(),
    };

    let path = format!("{}.{}", file_name, output_format.as_str());

    if let Err(error) = fs::write(&path, content) {
        eprintln!("[ERROR] Failed to write file {}: {}", path, error);
    }
}
