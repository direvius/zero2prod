use actix_web::{
    post,
    web::{Data, Form},
    HttpResponse, Responder,
};
use chrono::Utc;
use log::error;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(info: Form<Info>, pool: Data<PgPool>) -> impl Responder {
    match query!(
        r#"
        insert into subscribtions
        (id, name, email, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        info.name,
        info.email,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("failed to save subscribtion: {}", e);
            HttpResponse::InternalServerError().body(format!("failed to save subscribtion: {}", e))
        }
    }
}
