use actix_web::{
    post,
    web::{Data, Form},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing::{info, error, instrument};
use uuid::Uuid;

use crate::domain::NewSubscriber;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        Ok(NewSubscriber {
            email: value.email.try_into()?,
            name: value.name.try_into()?,
        })
    }
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
async fn subscribe(info: Form<FormData>, pool: Data<PgPool>) -> impl Responder {
    let new_subscriber = match info.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(err) => return HttpResponse::BadRequest().body(format!("invalid form data: {}", err)),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => {
            info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("failed to save subscribtion: {:?}", e);
            HttpResponse::InternalServerError().body(format!("failed to save subscribtion: {}", e))        }
    }
}

#[instrument(
    name = "Saving subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
            insert into subscribtions
            (id, name, email, subscribed_at)
            values ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.name.as_ref(),
        new_subscriber.email.as_ref(),
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
