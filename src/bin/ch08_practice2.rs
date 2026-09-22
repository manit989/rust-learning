fn safe_truncate(text: &str, max_chars: usize) -> String {
    // 1. Agar character count <= max_chars hai, toh as-is String return karo.
    // 2. Agar lamba hai, toh first `max_chars` characters lo aur end me "..." lagao.
    // DHYAN RAHE: Byte slicing &text[..max_chars] use nahi karna!
    if text.chars().count() <= max_chars {
        return text.to_string();
    }

    let mut truncated: String = text.chars().take(max_chars).collect();
    truncated.push_str("...");
    truncated
}

fn format_greentext(body: &str) -> String {
    // Lines traverse karo, '>' check karo, aur format karke ek single String me assemble karo
    body.lines()
        .map(|line| {
            if line.starts_with('>') {
                format!("[green]{line}[/green]")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_tripcode(name: &str, trip: Option<&str>) -> String {
    // Anonymous check karo, tripcode append karo
    if name.trim().is_empty() {
        String::from("Anonymous")
    } else {
        match trip {
            Some(code) => format!("{name}!{code}"),
            None => String::from(name),
        }
    }
}

fn main() {
    // --- Test 1: Safe Unicode Truncation ---
    let hindi_text = "नमस्ते दुनिया!"; // Har devanagari char 3 bytes ka hota hai
    let emoji_text = "Rust 🦀 is blazing fast!"; // 🦀 4 bytes ka hota hai

    println!("Truncated Hindi: {}", safe_truncate(hindi_text, 4));
    // Expected: "नमस्..." (Panic nahi hona chahiye!)

    println!("Truncated Emoji: {}", safe_truncate(emoji_text, 7));
    // Expected: "Rust 🦀 ..."

    println!("Short Text: {}", safe_truncate("Hello", 10));
    // Expected: "Hello"

    // --- Test 2: Greentext Formatter ---
    let post_body = ">be me\nlearning rust\n>compiler screams at me\nsadly drinks chai";
    let formatted = format_greentext(post_body);
    println!("\nFormatted Post:\n{}", formatted);
    /* Expected Output:
    [green]>be me[/green]
    learning rust
    [green]>compiler screams at me[/green]
    sadly drinks chai
    */

    // --- Test 3: Tripcode / Author Name ---
    println!("\nAnon: {}", generate_tripcode("   ", None));
    // Expected: "Anonymous"

    println!("Named: {}", generate_tripcode("Manit", None));
    // Expected: "Manit"

    println!(
        "Secured: {}",
        generate_tripcode("Manit", Some("4chanTrip#123"))
    );
    // Expected: "Manit!4chanTrip#123"
}
