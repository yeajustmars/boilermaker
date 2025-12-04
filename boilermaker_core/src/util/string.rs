pub fn truncate_to_char_count(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}
