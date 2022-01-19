use actix_web::{get, HttpResponse, Responder};

// #[derive(Deserialize, Serialize, Debug)]
// pub struct Info {
//     username: String,
//     email: String,
//     password: String,
//     confirm_password: String,
// }

#[get("/messages")]
pub async fn messages() -> impl Responder {
    HttpResponse::Ok().body("All messages:")
}

#[get("/message/{id}")]
pub async fn message_by_id() -> impl Responder {
    HttpResponse::Ok().body("Message: {id}")
}