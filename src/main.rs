use actix_web::{middleware::Logger, App, HttpServer};
use zero2prod::{configuration::read_configuration, services::{health_check, subscribe}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let configuration = read_configuration().expect("Failed to read configuration");
    HttpServer::new( || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health_check)
            .service(subscribe)
    })
    .bind(("127.0.0.1", configuration.app_port))?
    .run()
    .await
}
