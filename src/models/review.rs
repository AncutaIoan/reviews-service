use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct Review {
    id: String,
    product_id: String,
    added_by: String,
    added_at: String,
    rating: u8,
}

impl Review {
    pub(crate) fn new(product_id: String, added_by: String, added_at: String, rating: u8) -> Self {
        let id = Uuid::new_v4().to_string();
        Review {
            id,
            product_id,
            added_by,
            added_at,
            rating,
        }
    }
    fn validate(&self) -> bool {
        if self.product_id.is_empty()
            || self.added_by.is_empty()
            || self.added_at.is_empty()
            || self.rating > 5
        {
            return false;
        }

        true
    }
}
