use actix_web::{App, HttpServer};
use zero2prod::{hello, health_check, echo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(hello)
            .service(health_check)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
