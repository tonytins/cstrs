use substring::Substring;

pub fn get_entry(entires: Vec<String>, key: String) -> String {
    let CARET = '^';
    let LF = "\u{000A}";
    let CR = "\u{000D}";
    let CRLF = "\u{000D}\u{000A}";
    let LS = "\u{2028}";

    for entry in entires {
        if !entry.contains(&key) {
            continue;
        }

        // let start_index = entry.find(&key).chars().to_string();
        // let line = start_index.substring(start_index, entry.len());

        return "TBA".to_string();
    }

    return "***MISSING***".to_string();
}

struct UIText {}

impl UIText {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
