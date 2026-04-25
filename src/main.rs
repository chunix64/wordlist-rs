mod cli;
mod formatter;
mod parser;
mod utils;

use std::{process, time::Instant};

use crate::{
    parser::{parse_word_list, wordlist::get_word_list_size},
    utils::fs::{collect_files, save_word_list},
};
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    run(cli);
}

fn run(cli: Cli) {
    let start_time = Instant::now();
    let files = collect_files(&cli.paths, cli.recursive);

    if files.is_empty() {
        println!("[INFO] No file detected!");
        process::exit(0);
    }

    let word_list = parse_word_list(files);
    save_word_list(&word_list, &cli.output, &cli.output_format, &cli.sort);

    let (unique_words, total_words) = get_word_list_size(&word_list);
    let execution_time = start_time.elapsed().as_secs_f64();

    println!(
        "\n[INFO] {} words found with {} unique words in {}s",
        total_words, unique_words, execution_time
    );
}
