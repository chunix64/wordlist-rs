use scraper::Html;

pub fn extract_text_from_html(html: &str) -> String {
    let document = Html::parse_document(html);

    document.root_element().text().collect::<Vec<_>>().join("")
}

pub fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphabetic() || *c == '\'')
        .collect::<String>()
        .to_lowercase()
}
