use std::str::FromStr;

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::Uuid as SqlxUuid, PgPool};
use uuid::Uuid;

use crate::domain::NewSubscriber;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
  name = "Adding new subscriber", skip(form, db_conn),
  fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
  )
)]
pub async fn subscribe(form: web::Form<FormData>, db_conn: web::Data<PgPool>) -> HttpResponse {
    //

    let new_subscriber = match form.0.try_into() {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subscriber(&new_subscriber, &db_conn).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Persisting new subscriber", skip(new_subscriber, db_cp))]
async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    db_cp: &PgPool,
) -> Result<(), sqlx::Error> {
    let row_id = SqlxUuid::from_str(Uuid::new_v4().to_string().as_str()).unwrap();
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        row_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_cp)
    .await
    .map_err(|e| {
        match e.as_database_error() {
            Some(dbe) => {
                let dbe_code = dbe.code().unwrap_or_default();
                tracing::error!("Failed to execute query: {:?} (code={}).", e, dbe_code);
            }
            None => {
                tracing::error!("Failed to execute query: {:?}.", e);
            }
        }
        e
        // TODO: Use an App related error in case of a Db error.
        //   match dbe_code.into_owned().as_str() {
        //     "23505" => HttpResponse::Conflict().finish(),
        //     _ => HttpResponse::InternalServerError().finish(),
        //  }
    })?;
    Ok(())
}
