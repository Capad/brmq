use actix_web::{delete, HttpResponse, Responder};

#[delete("/discard/{id}")]
pub async fn discard(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}