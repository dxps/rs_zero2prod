use std::net::TcpListener;

use rs_zero2prod::{
    config::get_config,
    startup::run,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //
    let ts = get_tracing_subscriber("rs_ztp".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(ts);

    // Load the config and init db connection. Panic if this fails.
    let app_config = get_config().expect("Failed to load the app config.");

    let db_conn_pool = PgPool::connect(&app_config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let endpoint = format!("127.0.0.1:{}", app_config.http_port);
    let listener =
        TcpListener::bind(&endpoint).expect(&format!("Failed to listen on {}.", endpoint));

    run(listener, db_conn_pool)?.await
}
