use actix_web::{web, App, HttpServer};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use zero2prod::{
    configuration::read_configuration,
    services::{health_check, subscribe},
    telemetry::{get_subscriber, init_logging},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_logging(subscriber);
    let configuration = read_configuration().expect("Failed to read configuration");
    let db_string = configuration.database.connection_string();
    let pool = PgPool::connect(&db_string.expose_secret())
        .await
        .expect("Failed to connect to database");
    let pool = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(pool.clone())
    })
    .bind(("127.0.0.1", configuration.app_port))?
    .run()
    .await
}
