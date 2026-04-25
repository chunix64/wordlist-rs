mod json;
mod txt;
use crate::cli::{OutputFormat, SortOrder};
use json::JsonFormatter;
use std::collections::HashMap;
use txt::TxtFormatter;

pub trait Formatter {
    fn format(&self, word_list: &HashMap<String, u64>, sort_order: &SortOrder) -> String;
}

fn sort_word_list<'a>(
    word_list: &'a HashMap<String, u64>,
    sort_order: &SortOrder,
) -> Vec<(&'a String, &'a u64)> {
    let mut items: Vec<_> = word_list.iter().collect();

    match sort_order {
        SortOrder::None => {}
        SortOrder::Asc => {
            items.sort_by(|a, b| a.1.cmp(b.1).then_with(|| a.0.cmp(b.0)));
        }
        SortOrder::Desc => {
            items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        }
    }

    items
}

pub fn get_formatter(output_format: &OutputFormat) -> Box<dyn Formatter> {
    match output_format {
        OutputFormat::Txt => Box::new(TxtFormatter),
        OutputFormat::Json => Box::new(JsonFormatter),
    }
}
