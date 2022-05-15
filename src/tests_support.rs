use crate::config::DatabaseSettings;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{io::Error, net::TcpListener};
use uuid::Uuid;

pub struct TestApp {
    server_handle: tokio::task::JoinHandle<Result<(), Error>>,
    pub http_endpoint: String,
    pub db: PgPool,
}

impl TestApp {
    pub async fn stop_server(&self) {
        self.server_handle.abort();
    }
}

/// Spin up an instance of the app (incl. the web server)
/// and return the initialized elements (listening address and database connection pool).
pub async fn spawn_test_app_db() -> TestApp {
    // Load the config and init db connection. Panic if this fails.
    let mut app_config = crate::config::get_config().expect("Failed to load the app config.");
    app_config.database.name = Uuid::new_v4().to_string();
    let db = configure_database(&app_config.database).await;

    // Port value of 0 (in "{ip/name}:0") will trigger an OS scan for
    // an available port that can be used for binding (listening to).
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = crate::startup::run(listener, db.clone()).expect("Failed to bind address");
    // let _ = tokio::spawn(server);
    let server_handle = tokio::spawn(server);

    TestApp {
        server_handle,
        http_endpoint: format!("http://127.0.0.1:{}", port),
        db,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create the database.
    let mut conn = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database");

    // Run the database migrations.
    let conn_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to run the database migrations");

    conn_pool
}
