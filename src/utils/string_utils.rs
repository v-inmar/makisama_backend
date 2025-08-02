pub fn is_alphanumeric_or_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn is_first_character_underscore(s: &str) -> bool {
    if let Some(c) = s.chars().next() {
        c == '_'
    } else {
        false
    }
}
