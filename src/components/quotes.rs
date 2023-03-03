use std::collections::HashMap;

pub fn get_quote(word: &str) -> String {
    let quotes = HashMap::from([
        ("SANA","Lainaus"),
    ]);
    return quotes[word].to_string();
}

pub fn get_url(word: &str) -> String {
    let urls = HashMap::from([
        ("SANA","Url"),
    ]);
    return urls[word].to_string();
}