use serde::{Deserialize, Serialize};

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Serialize, Deserialize)]
pub struct Chan {
    pub id: String,
    pub label: String,
    pub unit: String,
    pub value: Option<f32>,
    pub description: String,
}


