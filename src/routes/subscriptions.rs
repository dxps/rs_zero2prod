use std::str::FromStr;

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::Uuid as SqlxUuid, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_conn: web::Data<PgPool>) -> HttpResponse {
    //
    let id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding new subscriber.", %id, subscriber_email = %form.email, subscriber_name = %form.name);
    let query_span = tracing::info_span!("Persisting new subscriber");
    let row_id = SqlxUuid::from_str(id.to_string().as_str()).unwrap();
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        row_id,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_conn.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("req_id {} - New subscriber saved.", id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            let dbe = e.as_database_error().unwrap();
            let dbe_code = dbe.code().unwrap_or_default();
            tracing::error!(
                "req_id {} - Failed to execute query: {:?} (code={}).",
                id,
                dbe,
                dbe_code
            );
            match dbe_code.into_owned().as_str() {
                "23505" => HttpResponse::Conflict().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}
