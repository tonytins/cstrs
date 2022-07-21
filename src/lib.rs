use normalize_line_endings::normalized;
use std::collections::TryReserveError;
use std::iter::FromIterator;
use substring::Substring;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const CARET: char = '^';

/// Converts the content into a vector of strings, each string being a line.
///
/// # Example
/// ```
/// let input = normalize_entries("1 ^The quick brown fox^\n2 ^jumps over the lazy dog^");
/// assert_eq!(input, vec!["1 ^The quick brown fox^", "2 ^jumps over the lazy dog^"]);
/// ```
fn normalize_entries(content: String) -> Vec<String> {
    let cst_ending = format!("{CARET}{LINE_ENDING}");
    let normalized_content = String::from_iter(normalized(content.chars()));
    let normalized_entries = normalized_content
        .split(cst_ending.as_str())
        .collect::<Vec<&str>>();

    return Vec::from_iter(normalized_entries.iter().map(|s| s.to_string()));
}

/// Get entries from a file and returns them as a string.
pub fn get_entry(content: String, key: usize) -> Result<String, TryReserveError> {
    let entries = normalize_entries(content);

    // Find the entry
    for entry in entries {
        if entry.contains(&key.to_string()) {
            let start_index = entry.find(CARET).unwrap();
            let line = entry.substring(start_index, entry.len());
            return Ok(line.to_string());
        }
    }

    // No entry found.
    return Ok("***MISSING***".to_string());
}

pub struct UIText {}

impl UIText {}

#[cfg(test)]
mod tests {
    use crate::normalize_entries;
    #[test]
    fn is_missing() {
        let input =
            normalize_entries("1 ^The quick brown fox^\n2 ^jumps over the lazy dog^".to_string());
        let expected = [
            "1 ^The quick brown fox^".to_string(),
            "2 ^jumps over the lazy dog^".to_string(),
        ];
        assert_eq!(input, expected);
    }
}
