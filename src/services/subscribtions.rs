use actix_web::{post, web::Form, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(info: Form<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} {}", info.email, info.name))
}
