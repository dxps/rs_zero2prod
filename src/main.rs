use std::net::TcpListener;

use rs_zero2prod::{config, startup};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the config and init db connection. Panic if this fails.
    let app_config = config::get_config().expect("Failed to load the app config.");
    let db_conn_pool = PgPool::connect(&app_config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let endpoint = format!("127.0.0.1:{}", app_config.http_port);
    let listener =
        TcpListener::bind(&endpoint).expect(&format!("Failed to listen on {}.", endpoint));

    startup::run(listener, db_conn_pool)?.await
}
