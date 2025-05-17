mod common;

use common::*;
use zero2prod::configuration::fixtures::*;

#[tokio::test]
#[rstest::rstest]
async fn configuration_none(app: App) {
    app.health_check().send().await.expect("App failed to start with no config");
}

#[tokio::test]
#[rstest::rstest]
async fn configuration_valid(#[with(Some(valid()))] app: App) {
    app.health_check().send().await.expect("App failed to start with valid config");
}

#[tokio::test]
#[rstest::rstest]
#[should_panic]
async fn configuration_invalid(#[with(Some(invalid()))] app: App) {
    app.health_check().send().await.expect_err("App should not be able to start with invalid configuration");
}
