use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .workers(4)
    // .bind_openssl("127.0.0.1:8080", builder)?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
