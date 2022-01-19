use actix_web::{get, HttpResponse, Responder};

#[get("/topics")]
pub async fn topics() -> impl Responder {
    HttpResponse::Ok().body("All topics:")
}

#[get("/topic/{id}")]
pub async fn topic_by_id() -> impl Responder {
    HttpResponse::Ok().body("Topic: {id}")
}