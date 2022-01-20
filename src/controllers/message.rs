use actix_web::{get, HttpResponse, Responder};

#[get("/message/{id}")]
pub async fn message() -> impl Responder {
    HttpResponse::Ok().body("msg")
}