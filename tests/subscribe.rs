use actix_web::{
    http::{header::ContentType, Method},
    middleware::Logger,
    test,
    web::{Data, ServiceConfig},
    App,
};
use maplit::hashmap;
use secrecy::ExposeSecret;
use sqlx::{query, Connection, Executor, PgConnection, PgPool};
use test_log::test as test_log;
use tracing::info;
use tracing_actix_web::TracingLogger;
use uuid::Uuid;
use zero2prod::{
    configuration::read_configuration,
    services::subscribe,
    telemetry::{get_subscriber, init_logging},
};

#[ctor::ctor]
fn init() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_logging(subscriber);
}

async fn db_init() -> Data<PgPool> {
    let mut configuration = read_configuration().expect("Failed to read configuration");
    configuration.database.db_name = Uuid::new_v4().to_string();
    info!("Creating DB: {}", configuration.database.db_name);
    PgConnection::connect(
        &configuration
            .database
            .connection_string_without_db()
            .expose_secret(),
    )
    .await
    .expect("Failed to connect to DB server")
    .execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database.db_name).as_str())
    .await
    .expect("Failed to create DB");
    let pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to open database");
    let pool = Data::new(pool);
    sqlx::migrate!("./migrations")
        .run(pool.get_ref())
        .await
        .expect("Failed to migrate");
    pool
}

fn config_app(pool: Data<PgPool>) -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool.clone()).service(subscribe);
    })
}

#[test_log(test)]
async fn test_returns_a_200_for_valid_form_data() {
    let pool = db_init().await;
    let app = test::init_service(
        App::new()
            .configure(config_app(pool))
            .wrap(TracingLogger::default()),
    )
    .await;
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/subscribe")
        .set_form(hashmap! {
            "name" => "le guin",
            "email" => "ursula_le_guin@gmail.com",
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[test_log(test)]
async fn test_returns_a_400_for_missing_form_data() {
    let pool = db_init().await;
    let app = test::init_service(
        App::new()
            .configure(config_app(pool))
            .wrap(Logger::default()),
    )
    .await;
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/subscribe")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[test_log(test)]
async fn test_inserts_user_into_a_database() {
    let pool = db_init().await;
    let app = test::init_service(
        App::new()
            .configure(config_app(pool.clone()))
            .wrap(Logger::default()),
    )
    .await;
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/subscribe")
        .set_form(hashmap! {
            "name" => "le guin",
            "email" => "ursula_le_guin@gmail.com",
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let saved = query!("select * from subscribtions")
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to read from table");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}
