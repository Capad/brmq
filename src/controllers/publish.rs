use actix_web::{post, HttpResponse, web, Error};

use crate::models::{publish_request::PublishRequest, publish_response::PublishResponse};

#[post("/publish")]
pub async fn publish(req: web::Json<PublishRequest>, res: web::Json<PublishResponse>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(res.0)) // <- send response
}