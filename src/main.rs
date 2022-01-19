use std::time::Duration;

use actix_web::{get, post, middleware::Logger, App, HttpResponse, HttpServer, Responder};
// use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use moka::future::Cache;

#[get("/messages")]
async fn messages() -> impl Responder {
    HttpResponse::Ok().body("All messages:")
}

#[get("/message/{id}")]
async fn message_by_id() -> impl Responder {
    HttpResponse::Ok().body("Message: {id}")
}

#[get("/topics")]
async fn topics() -> impl Responder {
    HttpResponse::Ok().body("All topics:")
}

#[get("/topic/{id}")]
async fn topic_by_id() -> impl Responder {
    HttpResponse::Ok().body("Topic: {id}")
}

#[get("/publishers")]
async fn publishers() -> impl Responder {
    HttpResponse::Ok().body("All publishers:")
}

#[get("/publisher/{id}")]
async fn publisher_by_id() -> impl Responder {
    HttpResponse::Ok().body("publisher: {id}")
}

#[post("/subscribe")]
async fn subscribe() -> impl Responder {
    HttpResponse::Ok().body("subscribe to publisher, topic or both")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let cache = Cache::builder()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(60 * 60))
        .time_to_idle(Duration::from_secs( 5 * 60))
        .build();

    // test
    cache.insert(1, "one".to_string()).await;

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(messages)
            .service(message_by_id)
            .service(topics)
            .service(topic_by_id)
            .service(publishers)
            .service(publisher_by_id)
            .service(subscribe)
    })
    .workers(4)
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
