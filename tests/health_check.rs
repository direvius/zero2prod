use actix_web::{body::to_bytes, http::header::ContentType, test, App};
use zero2prod::services::health_check;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::default()
        .uri("/health_check")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = resp.into_body();
    let bytes = to_bytes(body).await.unwrap();
    assert_eq!(bytes.len(), 0, "response body is not empty");
}
