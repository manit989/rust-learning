fn extract_parentheses(s: &str) -> Option<&str> {
    let mut fop: Option<usize> = None;
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'(' && fop.is_none() {
            fop = Some(i);
        } else if b == b')'
            && let Some(opi) = fop
        {
            return Some(&s[opi + 1..i]);
        }
    }

    None
}

fn main() {
    let text1 = "user_id: (4281), status: active";
    println!("Found: {:?}", extract_parentheses(text1));
    // Expected: Some("4281")

    let text2 = "empty tuple () in code";
    println!("Found: {:?}", extract_parentheses(text2));
    // Expected: Some("")

    let text3 = "unclosed (parenthesis here";
    println!("Found: {:?}", extract_parentheses(text3));
    // Expected: None

    let text4 = "inverted )first( check";
    println!("Found: {:?}", extract_parentheses(text4));
    // Expected: None

    let text5 = "no brackets at all";
    println!("Found: {:?}", extract_parentheses(text5));
    // Expected: None
}
