use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    // Initial arrangement.
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

fn spawn_app() -> String {
    // Port value of 0 (in "...:0") will trigger an OS scan for
    // an available port that can be used for binding (listening to).
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = rs_zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    // Returning the listening address.
    format!("http://127.0.0.1:{}", port)
}
