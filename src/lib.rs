use actix_web::{get, post, web::Form, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(info: Form<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} {}", info.email, info.name))
}
