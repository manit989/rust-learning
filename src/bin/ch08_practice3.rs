use std::collections::HashMap;

fn record_visit(ip_tracker: &mut HashMap<String, u32>, ip: &str) -> bool {
    // Entry API se count update karo, 5 se zyada hone par false return karo
    if let Some(count) = ip_tracker.get_mut(ip) {
        if *count < 5 {
            *count += 1;
            true
        } else {
            false
        }
    } else {
        ip_tracker.insert(ip.to_string(), 0);
        true
    }
}

fn aggregate_board_tags(posts: &[(&str, &str)]) -> HashMap<String, Vec<String>> {
    let mut board_tags: HashMap<String, Vec<String>> = HashMap::new();
    for (board, post) in posts {
        let vec_posts = board_tags.entry(board.to_string()).or_default();

        vec_posts.push(post.to_string());
    }
    board_tags
}

fn top_active_board(boards: &HashMap<String, Vec<String>>) -> Option<(&str, usize)> {
    // Sabse zyada posts wala board find karo
    boards
        .iter()
        .max_by_key(|(_, posts)| posts.len())
        .map(|(board, posts)| (board.as_str(), posts.len()))
}

fn main() {
    // --- Test 1: Rate Limiter ---
    let mut ip_hits: HashMap<String, u32> = HashMap::new();
    let client_ip = "192.168.1.50";

    for request_num in 1..=6 {
        let allowed = record_visit(&mut ip_hits, client_ip);
        println!("Request {}: Allowed? {}", request_num, allowed);
    }
    // Expected: Request 1..5: true, Request 6: false

    // --- Test 2: Board Posts Grouping ---
    let posts = vec![
        ("tech", "p101"),
        ("anime", "p102"),
        ("tech", "p103"),
        ("gaming", "p104"),
        ("tech", "p105"),
        ("anime", "p106"),
    ];

    let grouped = aggregate_board_tags(&posts);
    println!("\nGrouped boards:");
    for (board, p_ids) in &grouped {
        println!("{}: {:?}", board, p_ids);
    }

    // --- Test 3: Top Board ---
    if let Some((board, count)) = top_active_board(&grouped) {
        println!("\nMost active board is '{}' with {} posts.", board, count);
    }
    // Expected: Most active board is 'tech' with 3 posts.
}
