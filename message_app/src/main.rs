use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Message app")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet) // register the greet service
    })
    .bind(("127.0.0.1", 8080))? // bind the server to the address and port
    .run()
    .await
}