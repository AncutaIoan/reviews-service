use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub(crate) struct Review {
    id: i32,
    pub(crate) added_by: String,
    pub(crate) added_at: String,
    pub(crate) rating: i32,
    pub(crate) entity_type: String,  // Store entity_type as a string
    pub(crate) entity_id: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum EntityType {
    Product,
    Person,
    Company,
    Other(String),
}

impl EntityType {
    fn to_string(&self) -> String {
        match self {
            EntityType::Product => "Product".to_string(),
            EntityType::Person => "Person".to_string(),
            EntityType::Company => "Company".to_string(),
            EntityType::Other(s) => s.clone(),
        }
    }

    fn from_string(s: &str) -> EntityType {
        match s {
            "Product" => EntityType::Product,
            "Person" => EntityType::Person,
            "Company" => EntityType::Company,
            _ => EntityType::Other(s.to_string()),
        }
    }
}

impl Review {
    pub(crate) fn new(added_by: String, added_at: String, rating: i32, entity_type: EntityType, entity_id: String) -> Self {
        Review {
            id: 0,
            added_by,
            added_at,
            rating,
            entity_type: entity_type.to_string(),
            entity_id,
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
