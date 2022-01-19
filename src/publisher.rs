use actix_web::{get, HttpResponse, Responder};

#[get("/publishers")]
pub async fn publishers() -> impl Responder {
    HttpResponse::Ok().body("All publishers:")
}

#[get("/publisher/{id}")]
pub async fn publisher_by_id() -> impl Responder {
    HttpResponse::Ok().body("publisher: {id}")
}