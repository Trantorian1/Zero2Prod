mod common;

use ::logs::fixtures::*;
use common::*;

const HEADER: &str = "application/x-www-form-urlencoded";

#[tokio::test]
#[rstest::rstest]
async fn subscriptions_200(_logs: (), app: App) {
    let response = app
        .subscriptions()
        .header(reqwest::header::CONTENT_TYPE, HEADER)
        .form(&[("name", "Trantorian"), ("email", "trantorian@terminus.net")])
        .send()
        .await
        .expect("Failed to send subscription");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
#[rstest::rstest]
#[case::missing_email(Some("Trantorian"), None)]
#[case::missing_name(None, Some("trantorian@terminus.net"))]
#[case::missing_all(None, None)]
async fn subscriptions_400_missing_data(_logs: (), app: App, #[case] name: Option<&str>, #[case] email: Option<&str>) {
    let mut form = std::collections::HashMap::new();
    if let Some(name) = name {
        form.insert("name", name);
    }
    if let Some(email) = email {
        form.insert("email", email);
    }

    let response = app
        .subscriptions()
        .header(reqwest::header::CONTENT_TYPE, HEADER)
        .form(&form)
        .send()
        .await
        .expect("Failed to send subscription");

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
}
