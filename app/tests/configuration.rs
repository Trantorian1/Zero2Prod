mod common;

use ::app::configuration::fixtures::*;
use ::logs::fixtures::*;
use common::*;

#[tokio::test]
#[rstest::rstest]
async fn configuration_none(_logs: (), app: App) {
    app.health_check().send().await.expect("App failed to start with no config");
}

#[tokio::test]
#[rstest::rstest]
async fn configuration_valid(_logs: (), #[with(Some(valid()))] app: App) {
    app.health_check().send().await.expect("App failed to start with valid config");
}

#[tokio::test]
#[rstest::rstest]
#[should_panic]
async fn configuration_invalid(_logs: (), #[with(Some(invalid()))] app: App) {
    app.health_check().send().await.expect_err("App should not be able to start with invalid configuration");
}
