use actix_web::{
    post,
    web::{Data, Form},
    HttpResponse, Responder,
};
use chrono::Utc;
use tracing::{error, info, Instrument};
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
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %info.email,
        subscriber_name = %info.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber to database");
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            error!("failed to save subscribtion: {:?}", e);
            HttpResponse::InternalServerError().body(format!("failed to save subscribtion: {}", e))
        }
    }
}
