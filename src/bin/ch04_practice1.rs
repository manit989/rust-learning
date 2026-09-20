fn longest_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut word_start: Option<usize> = None;
    let mut res = &s[0..0];

    for (i, &b) in bytes.iter().enumerate() {
        if b != b' ' && word_start.is_none() {
            word_start = Some(i);
        } else if b == b' ' && word_start.is_some() {
            let start = word_start.unwrap();
            if (i - start) > res.len() {
                res = &s[start..i];
            }
            word_start = None;
        }
    }

    if let Some(start) = word_start
        && (s.len() - start) > res.len()
    {
        res = &s[start..];
    }

    res
}

fn main() {
    let sentence = String::from("Rust makes memory safety incredibly fun");
    let result = longest_word(&sentence);

    println!("The longest word is: '{}'", result);

    let tricky = "   hello   world   ";
    println!("Tricky output: '{}'", longest_word(tricky));
}
