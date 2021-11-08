use std::net::TcpListener;

use rs_zero2prod::config;
use sqlx::{Connection, PgConnection, PgPool};

#[actix_rt::test]
async fn health_check_test() {
    // Setup.
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.http_listening_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Setup.
    let app = spawn_app().await;
    let db_conn_string = config::get_config()
        .expect("Failed to load config")
        .database
        .connection_string();
    let mut db_conn = PgConnection::connect(&db_conn_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let body = "name=Joe%20Black&email=joe%40gmail.com";

    // Act.
    let response = client
        .post(&format!("{}/subscriptions", &app.http_listening_address))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut db_conn)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "joe@gmail.com");
    assert_eq!(saved.name, "Joe Black");
}

#[actix_rt::test]
async fn subscribe_returns_400_when_missing_data() {
    // Setup.
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Using a table-driven test.
    let test_cases = vec![
        ("name=Joe%20Black", "email missing"),
        ("email=joe@black.com", "name missing"),
        ("", "both email and name missing"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act.
        let response = client
            .post(format!("{}/subscriptions", &app.http_listening_address))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to post request");

        println!(
            "For body: \"{}\" got response: status:{}",
            invalid_body,
            response.status()
        );

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 when request body has {}",
            error_msg
        );
    }
}

pub struct TestApp {
    pub http_listening_address: String,
    pub db_connection_pool: PgPool,
}

/// Spin up an instance of the app (incl. the web server)
/// and return the initialized elements (listening address and database connection pool).
async fn spawn_app() -> TestApp {
    // Load the config and init db connection. Panic if this fails.
    let app_config = config::get_config().expect("Failed to load the app config.");
    let db_conn = PgPool::connect(&app_config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Port value of 0 (in "{ip/name}:0") will trigger an OS scan for
    // an available port that can be used for binding (listening to).
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server =
        rs_zero2prod::startup::run(listener, db_conn.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        http_listening_address: format!("http://127.0.0.1:{}", port),
        db_connection_pool: db_conn,
    }
}
