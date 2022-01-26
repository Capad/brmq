use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PublishResponse {
    message_id : u32
}