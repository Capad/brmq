use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublishRequest {
    publisher : String,
    topic : String,
    message : String
}