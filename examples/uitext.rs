use cst::UIText;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let ui_text = UIText::new("lorem");
    let mut entries = [1, 2, 3, 4];
    entries.shuffle(&mut thread_rng());
    for entry in entries {
        println!(
            "{}{}****************{}",
            ui_text.get_text(101, entry).unwrap(),
            LINE_ENDING,
            LINE_ENDING
        );
    }
}
