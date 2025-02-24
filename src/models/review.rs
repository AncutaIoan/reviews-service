use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Review {
    added_by: String,
    added_at: String,
    rating: u8,
    entity_type: EntityType,
    entity_id: String
}

#[derive(Serialize, Deserialize)]
pub(crate) enum EntityType {
    Product,
    Person,
    Company,
    Other(String)
}

impl Review {
    pub(crate) fn new(added_by: String, added_at: String, rating: u8, entity_type: EntityType, entity_id: String) -> Self {
        Review {
            added_by,
            added_at,
            rating,
            entity_type,
            entity_id
        }
    }
    fn validate(&self) -> bool {
        if self.entity_id.is_empty()
            || self.added_by.is_empty()
            || self.added_at.is_empty()
            || self.rating > 5
        {
            return false;
        }

        true
    }
}
