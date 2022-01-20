use actix_web::{delete, HttpResponse, Responder};

#[delete("/discard/{id}")]
pub async fn discard() -> impl Responder {
    HttpResponse::Ok().body("Message: {id}")
}