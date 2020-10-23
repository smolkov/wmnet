use serde::{Deserialize, Serialize};

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prop {
    pub path: String,
    pub value: String,
}


