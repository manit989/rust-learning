#[derive(Debug)]
enum Paginated<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug)]
struct ApiResponse<D, M> {
    data: D,
    meta: M,
}

// 1. Generic implementation for any D and M
impl<D, M> ApiResponse<D, M> {
    fn new(data: D, meta: M) -> Self {
        ApiResponse { data, meta }
    }

    fn data(&self) -> &D {
        &self.data
    }

    fn meta(&self) -> &M {
        &self.meta
    }

    // Method with its own generic parameter NewM
    fn with_new_meta<NewM>(self, new_meta: NewM) -> ApiResponse<D, NewM> {
        ApiResponse {
            data: self.data,
            meta: new_meta,
        }
    }
}

// 2. Concrete implementation specifically for <String, u32>
impl ApiResponse<String, u32> {
    fn status_message(&self) -> String {
        format!("Status {}: {}", self.meta, self.data)
    }
}

// Test metadata structs
#[derive(Debug)]
struct PaginationMeta {
    page: usize,
    total_pages: usize,
}

fn main() {
    // --- Test 1: Generic ApiResponse with Paginated Data ---
    let post_titles = vec![
        String::from("Axum 0.8 released"),
        String::from("Why NixOS is awesome"),
    ];
    let page_meta = PaginationMeta {
        page: 1,
        total_pages: 5,
    };

    let response = ApiResponse::new(Paginated::Multiple(post_titles), page_meta);
    println!("Response meta: {:?}", response.meta());

    // --- Test 2: Mixing Generics (Transforming Metadata Type) ---
    // Change metadata from PaginationMeta to a simple bool (e.g., is_cached: true)
    let cached_response = response.with_new_meta(true);
    println!("Transformed Response: {:?}", cached_response);
    println!("New meta (bool): {}", cached_response.meta());

    // --- Test 3: Concrete Implementation ---
    let error_response = ApiResponse::new(String::from("Post not found"), 404);
    println!("{}", error_response.status_message());
    // Expected: "Status 404: Post not found"

    // Note: Calling status_message() on cached_response will fail compilation
    // because its types are <Paginated<Vec<String>>, bool>, not <String, u32>!
}
