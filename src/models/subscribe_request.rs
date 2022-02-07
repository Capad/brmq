use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SubscribeRequest {
    publisher : String,
    topic : String
}