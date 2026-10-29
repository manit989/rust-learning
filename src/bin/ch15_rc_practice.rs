use std::rc::Rc;

// =========================================================================
// Write your structs and functions here from scratch:
// 1. struct SharedImage { hash: String, file_path: String, size_kb: usize }
// 2. struct Reply { id: u64, comment: String, attachment: Rc<SharedImage> }
// 3. impl Reply { fn new(id: u64, comment: &str, attachment: Rc<SharedImage>) -> Self }
// 4. fn attachment_usage(img: &Rc<SharedImage>) -> usize
// =========================================================================

struct SharedImage {
    hash: String,
    file_path: String,
    size_kb: usize,
}

struct Reply {
    id: u64,
    comment: String,
    attachment: Rc<SharedImage>,
}

impl Reply {
    fn new(id: u64, comment: &str, attachment: Rc<SharedImage>) -> Self {
        Reply {
            id,
            comment: comment.to_string(),
            attachment,
        }
    }
}

fn attachment_usage(img: &Rc<SharedImage>) -> usize {
    Rc::strong_count(img)
}

fn main() {
    // 1. Create the initial shared image on the heap wrapped in Rc
    let reaction_img = Rc::new(SharedImage {
        hash: String::from("sha256:9f86d081884c"),
        file_path: String::from("/static/media/ferris_think.webp"),
        size_kb: 420,
    });

    println!("Initial count: {}", attachment_usage(&reaction_img));
    assert_eq!(attachment_usage(&reaction_img), 1);

    // 2. First reply shares ownership of the image
    let reply_1 = Reply::new(101, "Wait, does this compile?", Rc::clone(&reaction_img));
    println!("Count after reply_1: {}", attachment_usage(&reaction_img));
    assert_eq!(attachment_usage(&reaction_img), 2);

    // 3. Inner scope with temporary replies sharing the same image
    {
        let reply_2 = Reply::new(102, "Yes, zero-cost abstraction!", Rc::clone(&reaction_img));
        let reply_3 = Reply::new(103, "Same reaction here.", Rc::clone(&reaction_img));

        println!(
            "Count inside inner scope: {}",
            attachment_usage(&reaction_img)
        );
        assert_eq!(attachment_usage(&reaction_img), 4);

        // Notice how we can read SharedImage fields directly through Reply's Rc via Deref!
        println!(
            "Reply #{} uses file: {} ({} KB)",
            reply_2.id, reply_2.attachment.file_path, reply_2.attachment.size_kb
        );
        println!("Reply #{} says: {}", reply_3.id, reply_3.comment);
    } // <-- reply_2 and reply_3 go out of scope and are dropped here!

    // 4. Verify count drops back down when owners go out of scope
    println!(
        "Count after inner scope ends: {}",
        attachment_usage(&reaction_img)
    );
    assert_eq!(attachment_usage(&reaction_img), 2);

    println!(
        "Reply #{} still valid with hash: {}",
        reply_1.id, reply_1.attachment.hash
    );
}
