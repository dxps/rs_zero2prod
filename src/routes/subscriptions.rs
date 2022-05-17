use std::str::FromStr;

use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::Uuid as SqlxUuid, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_conn: web::Data<PgPool>) -> HttpResponse {
    //
    let id = SqlxUuid::from_str(Uuid::new_v4().to_string().as_str()).unwrap();
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        id,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_conn.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            let dbe = e.as_database_error().unwrap();
            let dbe_code = dbe.code().unwrap_or_default();
            println!("Failed to execute query: {} (code={}).", &dbe, dbe_code);
            match dbe_code.into_owned().as_str() {
                "23505" => HttpResponse::Conflict().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}
