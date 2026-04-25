mod cli;
mod formatter;
mod parser;
mod utils;

use crate::{
    parser::parse_word_list,
    utils::fs::{collect_files, save_word_list},
};
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    run(cli);
}

fn run(cli: Cli) {
    let files = collect_files(&cli.paths, cli.recursive);
    let word_list = parse_word_list(files);
    save_word_list(&word_list, &cli.output, &cli.output_format, &cli.sort);
}
