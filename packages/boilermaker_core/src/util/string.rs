pub fn truncate_to_char_count(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}

pub fn string_to_option(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
