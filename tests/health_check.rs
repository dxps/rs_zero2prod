use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    // Setup.
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_req() {
    // Setup.
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Joe%20Black&email=joe%40gmail.com";

    // Act.
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_when_missing_data() {
    // Setup.
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Joe%20Black", "email missing"),
        ("email=joe@black.com", "name missing"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act.
        let response = client
            .post(format!("{}/subscriptions", &address))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to post request");
        
        assert_eq!(400, response.status().as_u16(),
         "The API did not fail with 400 when response body was {}", error_msg);
    }
}

/// Spin up an instance of the app (incl. the web server)
/// and return the listening address.
fn spawn_app() -> String {
    // Port value of 0 (in "{ip/name}:0") will trigger an OS scan for
    // an available port that can be used for binding (listening to).
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = rs_zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    // Returning the listening address.
    format!("http://127.0.0.1:{}", port)
}
