mod common;

use common::*;

#[tokio::test]
#[rstest::rstest]
async fn heatlh_check(app: App) {
    let response = app.health_check().send().await.expect("Failed to query health check");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
