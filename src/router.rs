use crate::controller::review_controller;
pub struct Router;

impl Router {
    pub fn new() -> Self {
        Router
    }

    pub fn handle_request(&self, request: &str) -> String {
        if request.starts_with("GET /review") {
            return review_controller::review();
        }
        if request.starts_with("POST /review") {
            return if let Some(body) = self.extract_body(request) {
                review_controller::add_review(body)
            } else {
                self.bad_request()
            }
        }

        self.not_found()
    }

    fn extract_body(&self, request: &str) -> Option<String> {
        // Try to find the point where the body starts, after the headers
        if let Some(body_start) = request.body() {
            // The body starts after the first empty line (CRLF-CRLF)
            let body = &request[body_start + 4..]; // Skip the CRLF-CRLF part
            Some(body.trim().to_string())  // Trim any leading/trailing whitespace
        } else {
            None  // No body found
        }
    }
    fn not_found(&self) -> String {
        format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\nRoute Not Found")
    }

    fn bad_request(&self) -> String {
        format!("HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/plain\r\n\r\nRoute Not Found")
    }
}
