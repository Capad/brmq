use serde::Serialize;

#[derive(Serialize)]
pub struct SubscribeResponse {
    messages: String // should be list or array of messages, don't know how yet
}