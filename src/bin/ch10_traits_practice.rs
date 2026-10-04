use std::fmt::format;

// 1. Define the Trait
trait RenderHtml {
    fn render_body(&self) -> String;

    // Default implementation
    fn render_card(&self) -> String {
        format!("<div class=\"card\"> {} </div>", self.render_body())
    }
}

// 2. Concrete types
struct Thread {
    id: u64,
    title: String,
    author: String,
}

struct Reply {
    id: u64,
    content: String,
}

// Implement RenderHtml for Thread
impl RenderHtml for Thread {
    fn render_body(&self) -> String {
        format!(
            "<h3>#{} - {}</h3><p>By: {}</p>",
            self.id, self.title, self.author
        )
    }
}

// Implement RenderHtml for Reply
impl RenderHtml for Reply {
    fn render_body(&self) -> String {
        format!("<p>>>{} {}</p>", self.id, self.content)
    }
}

// 3. Generic wrapper with Trait Bounds
struct Pinned<T> {
    item: T,
    reason: String,
}

impl<T: RenderHtml> RenderHtml for Pinned<T> {
    fn render_body(&self) -> String {
        format!("[PINNED: {}] {} ", self.reason, self.render_body())
    }
}

// 4. Standalone functions using traits
fn log_preview(item: &impl RenderHtml) {
    println!("{}", item.render_card());
}

fn render_feed<T: RenderHtml>(items: &[T]) -> String {
    items
        .iter()
        .map(|x| x.render_card())
        .collect::<Vec<String>>()
        .join("\n")
}

// System notice helper
struct SystemNotice {
    message: String,
}

impl RenderHtml for SystemNotice {
    fn render_body(&self) -> String {
        format!("<strong>SYSTEM: {}</strong>", self.message)
    }
}

fn system_banner(notice: &str) -> impl RenderHtml {
    SystemNotice {
        message: notice.to_string(),
    }
}

fn main() {
    // --- Test 1: Basic Trait Methods & Default Implementation ---
    let t = Thread {
        id: 101,
        title: String::from("Rust on NixOS setup"),
        author: String::from("Ferris"),
    };

    println!("--- Test 1: Single Thread Card ---");
    log_preview(&t);
    // Expected:
    // <div class="card"><h3>#101 - Rust on NixOS setup</h3><p>By: Ferris</p></div>

    // --- Test 2: Trait Bounds on Slices ---
    let replies = vec![
        Reply {
            id: 1,
            content: String::from("Just use flakes."),
        },
        Reply {
            id: 2,
            content: String::from("Agreed, flakes make it reproducible."),
        },
    ];

    println!("\n--- Test 2: Rendered Feed ---");
    let feed_html = render_feed(&replies);
    println!("{}", feed_html);
    // Expected 2 separate <div class="card"> elements joined by newline

    // --- Test 3: Generic Pinned Container ---
    let pinned_thread = Pinned {
        item: Thread {
            id: 42,
            title: String::from("Board Rules & Guidelines"),
            author: String::from("Mod"),
        },
        reason: String::from("Read before posting"),
    };

    println!("\n--- Test 3: Pinned Component ---");
    log_preview(&pinned_thread);
    // Expected:
    // <div class="card">[PINNED: Read before posting] <h3>#42 - Board Rules & Guidelines</h3><p>By: Mod</p></div>

    // --- Test 4: Returning impl Trait ---
    println!("\n--- Test 4: System Banner ---");
    let banner = system_banner("Server maintenance at midnight UTC");
    log_preview(&banner);
    // Expected:
    // <div class="card"><strong>SYSTEM: Server maintenance at midnight UTC</strong></div>
}
