use unicode_segmentation::UnicodeSegmentation;

fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

fn main() {
    let precomposed = "café";
    let decomposed = "cafe\u{0301}";
    let family = "👨\u{200D}👩\u{200D}👧"; // family: man-woman-girl, ZWJ-joined

    for s in [precomposed, decomposed, family] {
        println!(
            "{s:>10}  bytes={:>2}  chars={:>2}  graphemes={:>2}  reversed={}",
            s.len(),
            s.chars().count(),
            s.graphemes(true).count(),
            reverse(s),
        );
    }
}
