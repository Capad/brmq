use actix_web::{post, HttpResponse, Responder};

#[post("/publish")]
pub async fn publish() -> impl Responder {
    HttpResponse::Ok().body("All publishers:")
}