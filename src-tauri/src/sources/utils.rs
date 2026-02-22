use regex::Regex;

pub fn normalize_string(s: &str) -> String {
    let mut normalized = s.to_lowercase();

    let re = Regex::new(r"(?i)\s*[\(\[].*[\)\]]|\s*(-|:).*$").unwrap();
    normalized = re.replace_all(&normalized, "").to_string();

    if normalized.starts_with("the ") {
        normalized = normalized[4..].to_string();
    }

    normalized.chars().filter(|c| c.is_alphanumeric()).collect()
}
