use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use substring::Substring;
use walkdir::WalkDir;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const MISSING: &'static str = "***MISSING***";

const CARET: char = '^';

/// Converts the content into a vector of strings, each string being a line.
fn normalize_entries<S: Into<String>>(content: S) -> Vec<String> {
    let cst_ending = format!("{}{}", CARET, LINE_ENDING);
    let entries = content
        .into()
        .trim_end()
        .trim_end_matches(CARET)
        .split(cst_ending.as_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    return entries;
}

/// Parses a specific entry from a string.
///
/// # Example
/// ```rust
/// use cst::get_entry;
///
/// let example = "1 ^The quick brown fox jumps over the lazy dog.^";
/// let entry = get_entry(example, 1);
/// println!("{}", entry);
/// ```
pub fn get_entry<S: Into<String>>(content: S, key: usize) -> String {
    let entries = normalize_entries(content);

    // Find the entry
    for entry in entries {
        if entry.contains(&key.to_string()) {
            let start_index = entry.find(CARET).unwrap();
            let line = entry.substring(start_index + 1, entry.len());
            return line.to_string();
        }
    }

    // No entry found.
    return MISSING.to_string();
}

pub struct UIText {
    pub language: String,
}

impl UIText {
    pub fn new<S: Into<String>>(language: S) -> UIText {
        UIText {
            language: language.into(),
        }
    }

    pub fn get_text(&self, id: usize, key: usize) -> String {
        let language_dir = format!(
            "{}/uitext/{}.dir/",
            env::current_dir().unwrap().display(),
            self.language
        );

        for entry in WalkDir::new(&language_dir)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            let find_id = format!("_{}_", id);
            if f_name.find(&find_id) != None && f_name.ends_with(".cst") {
                let mut open_file =
                    match File::open(Path::new(format!("{}/{}", language_dir, f_name).as_str())) {
                        Err(_) => panic!("couldn't open {}", f_name),
                        Ok(file) => file,
                    };
                let mut contents = String::new();
                match open_file.read_to_string(&mut contents) {
                    Ok(_) => return get_entry(contents, key),
                    Err(why) => panic!("couldn't read {}: {}", f_name, why),
                }
            }
        }
        return MISSING.to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_entry, normalize_entries, UIText};

    #[cfg(windows)]
    const LINE_ENDING: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LINE_ENDING: &'static str = "\n";

    #[test]
    fn get_uitext() {
        let ui_text = UIText::new("lorem");
        let expected = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin ac dictum orci, at tincidunt nulla. Donec aliquet, eros non interdum posuere, ipsum sapien molestie nunc, nec facilisis libero ipsum et risus. In sed lorem vel ipsum placerat viverra.".to_string();
        dbg!(ui_text.get_text(101, 1));
        // assert_eq!(ui_text.get_text(101, 1), expected);
    }

    #[test]
    fn is_normalized() {
        let example = format!(
            "1 ^The quick brown fox^{}2 ^jumps over the lazy dog^{}",
            LINE_ENDING, LINE_ENDING
        );
        let input = normalize_entries(example);
        let expected = [
            "1 ^The quick brown fox".to_string(),
            "2 ^jumps over the lazy dog".to_string(),
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn test_entry() {
        let example = format!(
            "# comment{}1 ^The quick brown fox^{}2 ^jumps over the lazy dog^{}",
            LINE_ENDING, LINE_ENDING, LINE_ENDING
        );
        let expected = "jumps over the lazy dog".to_string();
        dbg!(get_entry(&example, 2));
        assert_eq!(get_entry(example, 2), expected);
    }
}
