use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use zero2prod::{
    configuration::get_configuration,
    services::{health_check, subscribe},
    telemetry::{get_subscriber, init_logging},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_logging(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = PgPool::connect_with(configuration.database.with_db())
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
    .bind((
        configuration.application.host,
        configuration.application.port,
    ))?
    .run()
    .await
}
