#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Setup.
    let app = rs_zero2prod::tests_support::spawn_test_app_db().await;

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
        .fetch_one(&app.db)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "joe@gmail.com");
    assert_eq!(saved.name, "Joe Black");
}

#[actix_rt::test]
async fn subscribe_returns_400_when_missing_data() {
    // Setup.
    let app = rs_zero2prod::tests_support::spawn_test_app_db().await;

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
            .post(format!("{}/subscriptions", &app.http_endpoint))
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
