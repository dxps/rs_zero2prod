#[actix_rt::test]
async fn health_check_test() {
    // Setup.
    let app = rs_zero2prod::tests_support::spawn_test_app_db().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.http_endpoint))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// pub struct TestApp {
//     pub http_endpoint: String,
//     pub db_pool: PgPool,
// }

// /// Spin up an instance of the app (incl. the web server)
// /// and return the initialized elements (listening address and database connection pool).
// async fn spawn_app() -> TestApp {
//     // Load the config and init db connection. Panic if this fails.
//     let mut app_config = config::get_config().expect("Failed to load the app config.");
//     app_config.database.name = Uuid::new_v4().to_string();
//     let db_conn = configure_database(&app_config.database).await;

//     // Port value of 0 (in "{ip/name}:0") will trigger an OS scan for
//     // an available port that can be used for binding (listening to).
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();

//     let server =
//         rs_zero2prod::startup::run(listener, db_conn.clone()).expect("Failed to bind address");

//     let _ = tokio::spawn(server);

//     TestApp {
//         http_endpoint: format!("http://127.0.0.1:{}", port),
//         db_pool: db_conn,
//     }
// }

// async fn configure_database(config: &DatabaseSettings) -> PgPool {
//     // Create the database.
//     let mut conn = PgConnection::connect(&config.connection_string_without_db())
//         .await
//         .expect("Failed to connect to Postgres");
//     conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
//         .await
//         .expect("Failed to create database");

//     // Run the database migrations.
//     let conn_pool = PgPool::connect(&config.connection_string())
//         .await
//         .expect("Failed to connect to Postgres");
//     sqlx::migrate!("./migrations")
//         .run(&conn_pool)
//         .await
//         .expect("Failed to run the database migrations");

//     conn_pool
// }
