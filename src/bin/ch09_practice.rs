use std::num::ParseIntError;

#[derive(Debug)]
enum PostError {
    EmptyTitle,
    TitleTooLong(usize),
    InvalidFileExtension(String),
    FileTooLarge { size_kb: usize, max_kb: usize },
    InvalidBoardId(String),
}

struct RawPostRequest<'a> {
    board_id_raw: &'a str,
    title: &'a str,
    file_name: &'a str,
    file_size_kb: usize,
}

#[derive(Debug)]
struct ValidatedPost {
    board_id: u32,
    title: String,
    file_name: String,
}

fn validate_board_id(raw: &str) -> Result<u32, PostError> {
    // raw ko parse karo; ParseIntError ko PostError::InvalidBoardId me badal kar `?` lagao
    let board_id = raw
        .parse::<u32>()
        .map_err(|_| PostError::InvalidBoardId(raw.to_string()))?;

    Ok(board_id)
}

fn validate_title(title: &str) -> Result<String, PostError> {
    // Empty aur max 50 chars validation
    let title = title.trim();
    if title.is_empty() || title.chars().count() > 50 {
        Err(PostError::TitleTooLong(title.chars().count()))
    } else {
        Ok(title.to_string())
    }
}

fn validate_image(filename: &str, size_kb: usize) -> Result<String, PostError> {
    // 5120 KB limit check aur .png, .jpg, .webp extension check
    if size_kb < 5121 {
        let idx = filename
            .rfind('.')
            .ok_or_else(|| PostError::InvalidFileExtension(filename.to_string()))?;

        let extension = &filename[idx..];

        match extension {
            ".png" | ".jpg" | ".webp" => Ok(filename.to_string()),
            _ => Err(PostError::InvalidFileExtension(filename.to_string())),
        }
    } else {
        Err(PostError::FileTooLarge {
            size_kb,
            max_kb: 5120,
        })
    }
}

fn process_post(req: &RawPostRequest) -> Result<ValidatedPost, PostError> {
    // `?` operator ka use karke teeno validators chain karo
    Ok(ValidatedPost {
        board_id: validate_board_id(req.board_id_raw)?,
        title: validate_title(req.title)?,
        file_name: validate_image(req.file_name, req.file_size_kb)?,
    })
}

fn main() {
    let test_cases = vec![
        RawPostRequest {
            board_id_raw: "b", // Invalid board ID
            title: "Anon discussion",
            file_name: "pepe.png",
            file_size_kb: 1024,
        },
        RawPostRequest {
            board_id_raw: "42",
            title: "   ", // Empty title
            file_name: "ferris.jpg",
            file_size_kb: 500,
        },
        RawPostRequest {
            board_id_raw: "42",
            title: "Super legit post",
            file_name: "script.sh", // Invalid extension
            file_size_kb: 10,
        },
        RawPostRequest {
            board_id_raw: "42",
            title: "High res wallpaper",
            file_name: "image.webp",
            file_size_kb: 8000, // File too large
        },
        RawPostRequest {
            board_id_raw: "101",
            title: "Valid Post Title",
            file_name: "crab.png",
            file_size_kb: 2048, // All valid!
        },
    ];

    for (idx, req) in test_cases.iter().enumerate() {
        match process_post(req) {
            Ok(post) => println!("Case {}: Success! Created {:?}", idx + 1, post),
            Err(e) => println!("Case {}: Failed with error: {:?}", idx + 1, e),
        }
    }
}
