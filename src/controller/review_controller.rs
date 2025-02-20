use crate::models::review::Review;

pub fn hello() -> String {
    let review = Review::new(
        "Product123".to_string(),
        "JohnDoe".to_string(),
        "2025-02-20".to_string(),
        5,
    );

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        serde_json::to_string(&review).unwrap()
    )
}
