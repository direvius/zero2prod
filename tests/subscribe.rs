use actix_web::{http::{header::ContentType, Method}, test, App};
use maplit::hashmap;
use zero2prod::subscribe;

#[actix_web::test]
async fn test_returns_a_200_for_valid_form_data() {
    let app = test::init_service(App::new().service(subscribe)).await;
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/subscribe")
        .set_form(hashmap!{
            "name" => "le guin",
            "email" => "ursula_le_guin@gmail.com",
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_returns_a_400_for_missing_form_data() {
    let app = test::init_service(App::new().service(subscribe)).await;
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/subscribe")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}