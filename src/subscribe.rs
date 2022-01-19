use actix_web::{post, HttpResponse, Responder};

#[post("/subscribe")]
pub async fn subscribe() -> impl Responder {
    HttpResponse::Ok().body("subscribe to publisher, topic or both")
}