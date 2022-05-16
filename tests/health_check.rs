#[actix_rt::test]
async fn health_check_test() {
    // Setup.
    let mut app = rs_zero2prod::tests_support::TestApp::startup().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.http_endpoint))
        .send()
        .await
        .expect("Failed to execute request.");

    app.shutdown().await;

    // Evaluate
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
