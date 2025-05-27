mod common;

use common::*;

use ::logs::fixtures::*;

#[tokio::test]
#[rstest::rstest]
async fn heatlh_check(_logs: (), app: App) {
    let response = app.health_check().send().await.expect("Failed to query health check");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
