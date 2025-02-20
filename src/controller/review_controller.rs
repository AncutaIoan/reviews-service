use crate::models::review::Review;

pub fn review() -> String {
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


//TODO fixed trailing characters, maybe refactor to have a standard body instead of a string buffer

pub fn add_review(review_json: String) -> String {
    // Log the raw review JSON to see if there are extra characters
    println!("Received review JSON: {:?}", review_json);

    // Try trimming the review to remove any leading/trailing whitespace or unwanted characters
    let review_json = review_json.trim().to_string();

    // Log the trimmed JSON for debugging purposes
    println!("Trimmed review JSON: {:?}", review_json);

    // Attempt to deserialize the review JSON string
    let review: Result<Review, _> = serde_json::from_str(&review_json);

    match review {
        Ok(review) => {
            // Serialize the review back into JSON and send a successful response
            let response_json = serde_json::to_string(&review).unwrap();
            println!("Saved new review: {}", response_json);

            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                response_json
            )
        }
        Err(e) => {
            // Log the error for debugging
            eprintln!("Failed to parse review: {:?}", e);

            format!(
                "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/plain\r\n\r\nFailed to parse review: {:?}",
                e
            )
        }
    }
}

