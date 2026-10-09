use std::fmt::Display;

// 1. Function with explicit lifetime
fn pick_longer_comment<'a>(comm1: &'a str, comm2: &'a str) -> &'a str {
    // Longer string slice return karo
    if comm1.len() > comm2.len() {
        comm1
    } else {
        comm2
    }
}

// 2. Struct holding references
#[derive(Debug)]
struct PostSlice<'a> {
    board: &'a str,
    content: &'a str,
}

impl<'a> PostSlice<'a> {
    // Elision rule: output reference gets lifetime of &self automatically
    fn board_name(&self) -> &'a str {
        self.board
    }

    // Explicit lifetime: return value can come from either self or default_content
    fn content_or_default(&self, default_content: &'a str) -> &'a str {
        if self.content.trim().is_empty() {
            default_content
        } else {
            self.content
        }
    }
}

// 3. Combining Lifetimes, Generics, and Trait Bounds
fn display_and_pick<'a, T>(first: &'a str, second: &'a str, label: T) -> &'a str
where
    T: Display,
{
    // Label print karo aur longer string return karo
    println!("Comparing [{}]:",label);
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    // --- Test 1: Function Lifetime Contract ---
    let comment_a = String::from("Axum handlers are ergonomic!");
    let comment_b = String::from("Rust lifetimes make sense now.");

    let longer = pick_longer_comment(&comment_a, &comment_b);
    println!("Longer comment: {}", longer);

    // --- Test 2: Struct holding references and methods ---
    let board_raw = String::from("prog");
    let post_body = String::from("   "); // Empty whitespace post
    let fallback = "No content provided by anon.";

    let post = PostSlice {
        board: &board_raw,
        content: &post_body,
    };

    println!("\nBoard: /{}", post.board_name());
    println!("Final Content: {}", post.content_or_default(fallback));
    // Expected: Fallback print hona chahiye

    let valid_body = String::from("Compiles on NixOS cleanly!");
    let active_post = PostSlice {
        board: &board_raw,
        content: &valid_body,
    };
    println!(
        "Active Post Content: {}",
        active_post.content_or_default(fallback)
    );
    // Expected: "Compiles on NixOS cleanly!"

    // --- Test 3: Grand Finale (Lifetimes + Generics + Traits) ---
    println!();
    let winner = display_and_pick(&comment_a, &valid_body, "THREAD_VERIFICATION_TAG_#404");
    println!("Winner: {}", winner);

    // --- Test 4: Lifetime Safety Proof (Think & Observe) ---
    // Niche diye code ko uncomment karo aur dekho compiler dangling pointer kaise rokta hai:
    /*
    let outer_result;
    let string_one = String::from("Permanent thread");
    {
        let string_two = String::from("Temporary reply");
        outer_result = pick_longer_comment(&string_one, &string_two);
    } // string_two yahan drop ho gaya!
    println!("Result outside scope: {}", outer_result);
    */
}
