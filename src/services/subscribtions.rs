use actix_web::{
    post,
    web::{Data, Form},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

#[post("/subscribe")]
#[instrument(
    name = "Adding a new subscriber",
    skip(info, pool),
    fields(
        subscriber_email = %info.email,
        subscriber_name = %info.name
    )
)]
async fn subscribe(info: Form<Info>, pool: Data<PgPool>) -> impl Responder {
    match insert_subscriber(&pool, &info).await {
        Ok(_) => {
            info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("failed to save subscribtion: {:?}", e);
            HttpResponse::InternalServerError().body(format!("failed to save subscribtion: {}", e))
        }
    }
}

#[instrument(name = "Saving subscriber details in the database", skip(info, pool))]
pub async fn insert_subscriber(pool: &PgPool, info: &Info) -> Result<(), sqlx::Error> {
    query!(
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
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to save subscribtion: {:?}", e);
        e
    })?;
    Ok(())
}
