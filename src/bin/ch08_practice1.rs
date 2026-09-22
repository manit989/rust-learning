fn clean_tags(tags: &mut Vec<String>) {
    // 1. Tags ko trim aur lowercase karo (in-place)
    // 2. Sirf valid tags rakho jo '#' se shuru hote hain aur length > 1 ho
    for tag in tags.iter_mut() {
        *tag = tag.trim().to_lowercase();
    }

    tags.retain(|x| x.starts_with('#') && x.len() > 1);
}

fn safe_get_post(posts: &[String], index: usize) -> Option<&str> {
    // `.get()` ka use karke safe reference return karo
    posts.get(index).map(|x| x.as_str())
}

fn dedup_consecutive(numbers: &mut Vec<i32>) {
    // In-place duplicate elements remove karo
    numbers.dedup();
}

fn main() {
    // --- Test 1: Tag Sanitizer ---
    let mut tags = vec![
        String::from("  #RustLang "),
        String::from("  "),
        String::from("#axum"),
        String::from("spam_no_hash"),
        String::from("#"),
        String::from("#WebDev  "),
    ];

    clean_tags(&mut tags);
    println!("Cleaned tags: {:?}", tags);
    // Expected: ["#rustlang", "#axum", "#webdev"]

    // --- Test 2: Safe Fetch ---
    let posts = vec![String::from("First post"), String::from("Second post")];

    match safe_get_post(&posts, 1) {
        Some(content) => println!("Post at index 1: {}", content),
        None => println!("Index 1 not found!"),
    }

    match safe_get_post(&posts, 5) {
        Some(content) => println!("Post at index 5: {}", content),
        None => println!("Index 5 safely caught out-of-bounds!"),
    }

    // --- Test 3: Deduplication ---
    let mut nums = vec![1, 1, 2, 2, 2, 3, 1, 1, 4];
    dedup_consecutive(&mut nums);
    println!("Deduped numbers: {:?}", nums);
    // Expected: [1, 2, 3, 1, 4]

    // --- Test 4: Borrow Checker Trap (Think & Observe) ---
    // niche diye code ko uncomment karo aur dekho compiler kyun chillata hai:
    /*
    let mut feed = vec![10, 20, 30];
    let first = &feed[0];
    feed.push(40);
    println!("First element is: {}", first);
    */
}
