use std::time::Duration;
use actix_web::{post, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use controllers::{message::message, subscribe::subscribe, publish::publish, discard::discard};
// use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use moka::future::Cache;

mod controllers;
mod models;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    openssl_probe::init_ssl_cert_env_vars();

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
            .service(message)
            .service(subscribe)
            .service(publish)
            .service(discard)
    })
    .workers(4)
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
