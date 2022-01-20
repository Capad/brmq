use actix_web::{get, HttpResponse, Responder};

#[get("/subscribe")]
pub async fn subscribe() -> impl Responder {
    HttpResponse::Ok().body("subscribe to publisher, topic or both")
}