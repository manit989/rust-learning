use std::mem::size_of;

// 1. Recursive Data Structure
#[derive(Debug)]
enum QuoteChain {
    Root(String),
    Reply {
        id: u64,
        text: String,
        parent: Box<QuoteChain>, // Box ke bina 'recursive type has infinite size' error aayega
    },
}

// 2. Traversal Functions
fn chain_depth(chain: &QuoteChain) -> usize {
    // Chain ki total depth calculate karo
    match chain {
        QuoteChain::Root(_) => 1,
        QuoteChain::Reply {
            id: _,
            text: _,
            parent,
        } => 1 + chain_depth(parent),
    }
}

fn render_chain(chain: &QuoteChain) -> String {
    // Parent ko pehle render karo, fir current reply ko newline ke sath append karo
    match chain {
        QuoteChain::Root(vaad) => vaad.to_string(),
        QuoteChain::Reply { id, text, parent } => {
            format!("{}\n>> [#{id}]{text}", render_chain(parent))
        }
    }
}

// 3. Stack vs Heap Optimization
struct HeavyExifData {
    raw_bytes: [u8; 1024 * 1024], // 1 MB array
}

struct Post {
    id: u64,
    title: String,
    // Agar direct 'Option<HeavyExifData>' hota, toh Post ka stack size > 1MB ho jata!
    exif: Option<Box<HeavyExifData>>,
}

fn main() {
    // --- Test 1: Recursive Quote Chain ---
    let thread = QuoteChain::Root(String::from("Rust 2026 edition roadmap announced!"));

    let reply_1 = QuoteChain::Reply {
        id: 101,
        text: String::from("Will it stabilize custom allocators?"),
        parent: Box::new(thread),
    };

    let reply_2 = QuoteChain::Reply {
        id: 102,
        text: String::from("Yes, check the RFC draft."),
        parent: Box::new(reply_1),
    };

    println!("Total Chain Depth: {}", chain_depth(&reply_2));
    // Expected: 3

    println!("\nRendered Thread History:\n{}", render_chain(&reply_2));
    /* Expected Output:
    >> [OP]: Rust 2026 edition roadmap announced!
    >> [#101]: Will it stabilize custom allocators?
    >> [#102]: Yes, check the RFC draft.
    */

    // --- Test 2: Stack Size Verification ---
    println!("\n--- Memory Size Inspection ---");
    println!(
        "Size of HeavyExifData: {} bytes",
        size_of::<HeavyExifData>()
    );
    println!(
        "Size of Box<HeavyExifData>: {} bytes",
        size_of::<Box<HeavyExifData>>()
    );
    println!("Size of Post struct on Stack: {} bytes", size_of::<Post>());

    // Verify Post contains heap-allocated heavy data safely
    let large_post = Post {
        id: 1,
        title: String::from("High-res anime wallpaper"),
        exif: Some(Box::new(HeavyExifData {
            raw_bytes: [0u8; 1024 * 1024],
        })),
    };

    println!(
        "\nPost ID #{} successfully allocated with heavy exif on heap!",
        large_post.id
    );
}
