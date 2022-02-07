use serde::Serialize;

use super::message::Message;

#[derive(Serialize)]
pub struct SubscribeResponse {
    messages: Vec<Message>
}