use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_conn: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
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
