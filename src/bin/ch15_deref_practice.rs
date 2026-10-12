use std::ops::Deref;

// =========================================================================
// Write your struct and `impl Deref` here from scratch:
// 1. struct Sanitized<T>...
// 2. impl<T> Sanitized<T> { fn new(...) }
// 3. impl<T> Deref for Sanitized<T> ...
// =========================================================================

struct Sanitized<T>(T);

impl<T> Deref for Sanitized<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Sanitized<T> {
    fn new(value: T) -> Self {
        Sanitized(value)
    }
}

// Helper function expecting a plain string slice &str
fn inspect_clean_text(text: &str) {
    println!("Inspecting [len={}]: \"{}\"", text.len(), text);
}

fn main() {
    // --- Test 1: Explicit Dereference Operator (*) ---
    let post_score = Sanitized::new(42);
    // '*' operator should trigger deref() and yield the inner i32
    let updated_score = *post_score + 10;
    println!("Updated score: {}", updated_score);
    assert_eq!(updated_score, 52);

    // --- Test 2: Method Dispatch via Deref ---
    let post_title = Sanitized::new(String::from("   axum + maud guide   "));
    // Calling .trim() and .to_uppercase() directly on Sanitized without .0
    let formatted = post_title.trim().to_uppercase();
    println!("Formatted title: {}", formatted);
    assert_eq!(formatted, "AXUM + MAUD GUIDE");

    // --- Test 3: Deref Coercion (&Sanitized<String> -> &str) ---
    let clean_body = Sanitized::new(String::from("Post verified against XSS attacks."));
    // Passing &Sanitized<String> directly into fn(&str)
    // Coercion chain: &Sanitized<String> -> &String -> &str
    inspect_clean_text(&clean_body);
}
