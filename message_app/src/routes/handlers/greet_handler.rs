use actix_web::{get, HttpResponse, Responder};

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Message app")
}
