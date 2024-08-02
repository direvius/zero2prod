use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use zero2prod::{
    configuration::read_configuration,
    services::{health_check, subscribe},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let configuration = read_configuration().expect("Failed to read configuration");
    let db_string = configuration.database.connection_string();
    let pool = PgPool::connect(&db_string)
        .await
        .expect("Failed to connect to database");
    let pool = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health_check)
            .service(subscribe)
            .app_data(pool.clone())
    })
    .bind(("127.0.0.1", configuration.app_port))?
    .run()
    .await
}
