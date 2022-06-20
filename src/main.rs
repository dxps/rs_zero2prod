use secrecy::ExposeSecret;
use std::net::TcpListener;

use rs_zero2prod::{
    config::get_config,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //
    let ts = get_tracing_subscriber("rs_ztp".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(ts);

    // Load the config.
    let app_cfg = get_config().expect("Failed to load the app config.");

    // Init db connection pool.
    let db_conn_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(3))
        .connect_lazy(&app_cfg.database.connection_string().expose_secret())
        .expect("Failed to connect to database");
    // Init email client.
    let sender_email = app_cfg
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(app_cfg.email_client.api_base_url, sender_email);

    let endpoint = format!("{}:{}", app_cfg.http.host, app_cfg.http.port);
    let listener =
        TcpListener::bind(&endpoint).expect(&format!("Failed to listen on {}.", endpoint));

    run(listener, db_conn_pool, email_client)?.await
}
