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
/// let expected = [
/// "1 ^The quick brown fox".to_string(),
/// "2 ^jumps over the lazy dog^".to_string(),
/// ];
/// assert_eq!(input, expected);
/// ```
fn normalize_entries<S: Into<String>>(content: S) -> Vec<String> {
    let cst_ending = format!("{}{}", CARET, LINE_ENDING);
    let entries = content
        .into()
        .trim_end()
        .split(cst_ending.as_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    return entries;
}

/// Get entries from a file and returns them as a string.
pub fn get_entry<S: Into<String>>(content: S, key: usize) -> String {
    let entries = normalize_entries(content);

    // Find the entry
    for entry in entries {
        if entry.contains(&key.to_string()) {
            let start_index = entry.find(CARET).unwrap();
            let line = entry.substring(start_index, entry.len());
            return line.to_string();
        }
    }

    // No entry found.
    return "***MISSING***".to_string();
}

pub struct UIText {}

impl UIText {}

#[cfg(test)]
mod tests {
    use crate::normalize_entries;

    #[cfg(windows)]
    const LINE_ENDING: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LINE_ENDING: &'static str = "\n";

    #[test]
    fn is_normalized() {
        let example = format!(
            "1 ^The quick brown fox^{}2 ^jumps over the lazy dog^{}",
            LINE_ENDING, LINE_ENDING
        );
        let input = normalize_entries(example);
        dbg!(format!("{:?}", input));
        let expected = [
            "1 ^The quick brown fox".to_string(),
            "2 ^jumps over the lazy dog^".to_string(),
        ];
        assert_eq!(input, expected);
    }
}
