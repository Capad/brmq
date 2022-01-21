use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    publisher: String,
    id: u32,
    topic: String,
    body: String,
}
