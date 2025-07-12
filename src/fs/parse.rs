pub fn checked_string_to_f64(s: String) -> Option<f64> {
    let trimmed = s.as_str().trim_matches('"');
    trimmed.parse::<f64>().ok()
}
