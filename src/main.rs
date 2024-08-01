use actix_web::{middleware::Logger, App, HttpServer};
use zero2prod::{echo, health_check, hello, subscribe};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new( || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(health_check)
            .service(echo)
            .service(subscribe)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
