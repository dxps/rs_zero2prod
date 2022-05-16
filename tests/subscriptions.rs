#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Setup.
    let mut app = rs_zero2prod::tests_support::TestApp::startup().await;

    let client = reqwest::Client::new();
    let body = "name=Joe%20Black&email=joe%40gmail.com";

    // Act.
    let response = client
        .post(&format!("{}/subscriptions", &app.http_endpoint))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    app.shutdown().await;

    // Evaluate
    assert_eq!(saved.email, "joe@gmail.com");
    assert_eq!(saved.name, "Joe Black");
}

#[actix_rt::test]
async fn subscribe_returns_400_when_missing_data() {
    // Setup.
    let mut app = rs_zero2prod::tests_support::TestApp::startup().await;

    let client = reqwest::Client::new();

    // Using a table-driven test.
    let mut test_cases = vec![
        ("name=Joe%20Black", "email missing", 0_u16),
        ("email=joe@black.com", "name missing", 0_u16),
        ("", "both email and name missing", 0_u16),
    ];

    for (invalid_body, _, test_response_status) in &mut test_cases {
        // Act.
        let response = client
            .post(format!("{}/subscriptions", &app.http_endpoint))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(*invalid_body)
            .send()
            .await
            .expect("Failed to post request");
        // Capture the response.
        *test_response_status = response.status().as_u16();
    }

    app.shutdown().await;

    // Evaluate
    for (_, test_desc, test_response_status) in test_cases {
        assert_eq!(
            400, test_response_status,
            "The API did not fail with 400 when request body has {}",
            test_desc
        );
    }
}
