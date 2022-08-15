pub fn empty_to_none(s: Option<String>) -> Option<String> {
    match s {
        Some(v) if v.is_empty() => None,
        Some(v) => Some(v),
        None => None,
    }
}
