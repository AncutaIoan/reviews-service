use crate::controller::review_controller;
pub struct Router;

impl Router {
    pub fn new() -> Self {
        Router
    }

    pub fn handle_request(&self, request: &str) -> String {
        if request.starts_with("GET /hello") {
            return review_controller::hello();
        }
        self.not_found()
    }

    fn not_found(&self) -> String {
        format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\nRoute Not Found")
    }
}
