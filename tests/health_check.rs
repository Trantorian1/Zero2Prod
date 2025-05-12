mod common;

#[tokio::test]
async fn heatlh_check() {
    let address = common::spawn();

    let client = reqwest::Client::new();
    let response = client.get(format!("{address}/health_check")).send().await.expect("Failed to query health check");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
