use crate::config::DatabaseSettings;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{io::Error, net::TcpListener};
use uuid::Uuid;

pub struct TestApp {
    pub http_endpoint: String,
    pub db_pool: PgPool,
    db_conn: PgConnection,
    db_name: String,
    server_handle: tokio::task::JoinHandle<Result<(), Error>>,
}

impl TestApp {
    /// It spins up an instance of `TestApp` (incl. web server and database conn pool).
    pub async fn startup() -> Self {
        // Load the config and init db connection. Panic if this fails.
        let mut app_config = crate::config::get_config().expect("Failed to load the app config.");
        // Each spawned test app uses its own (unique per execution) database.
        let db_name = Uuid::new_v4().to_string();
        app_config.database.name = db_name.clone();
        let (db_conn, db_pool) = Self::configure_database(&app_config.database).await;

        // Port value of 0 (in "{ip/name}:0") will trigger an OS scan for
        // an available port that can be used for binding (listening to).
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();

        let server =
            crate::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
        let server_handle = tokio::spawn(server);

        Self {
            server_handle,
            http_endpoint: format!("http://127.0.0.1:{}", port),
            db_pool,
            db_conn,
            db_name,
        }
    }

    async fn configure_database(config: &DatabaseSettings) -> (PgConnection, PgPool) {
        // Create the database.
        let mut conn = PgConnection::connect(&config.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");
        conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
            .await
            .expect("Failed to create database");
        println!("[TestApp.startup] Created database {}.", config.name);

        // Run the database migrations.
        let conn_pool = PgPool::connect(&config.connection_string())
            .await
            .expect("Failed to connect to Postgres");
        sqlx::migrate!("./migrations")
            .run(&conn_pool)
            .await
            .expect("Failed to run the database migrations");

        (conn, conn_pool)
    }

    /// It performs a graceful shutdown: it stops the web server and
    /// removes the database being used in the current instance.
    pub async fn shutdown(&mut self) {
        // Shutdown web server thread.
        self.server_handle.abort();
        // Remove the database created at its spawning time.
        self.db_pool.close().await;
        match self
            .db_conn
            .execute(format!(r#"DROP DATABASE "{}";"#, self.db_name).as_str())
            .await
        {
            Ok(_) => println!("[TestApp.shutdown] Removed database {}.", self.db_name),
            Err(e) => {
                let dbe = e.as_database_error().unwrap();
                let dbe_code = dbe.code().unwrap_or_default();
                println!(
                    "[TestApp.shutdown] Failed to remove database: {} (code={}).",
                    &dbe, dbe_code
                );
            }
        };
    }
}
