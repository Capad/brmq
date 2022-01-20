use actix_web::{post, HttpResponse, Responder};

#[post("/publish")]
pub async fn publish(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}