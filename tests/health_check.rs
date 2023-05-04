use reqwest;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // use `reqwest` to make HTTP requests to our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");
    // launch as background task 
    // tokio::spawn returns a handle to the spawned future
    // no use here so we have non-binding let
    let _ = tokio::spawn(server);
}