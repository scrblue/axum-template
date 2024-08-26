use http::StatusCode;
use serde_json::json;

use self::util::{
    random_email, TestServer, ROUTE_ACCOUNT_LOG_IN, ROUTE_ACCOUNT_REGISTER,
    USER_ACCOUNT_TEST_DISPLAY_NAME, USER_ACCOUNT_TEST_PASS_INVALID, USER_ACCOUNT_TEST_PASS_VALID,
};

mod util;

/// Tests the happy path of registration and logging in.
#[tokio::test]
async fn test_register_and_log_in() {
    let server = TestServer::start().await;
    let client = reqwest::Client::new();

    let base_uri = server.base_uri();
    let register_uri = format!("{base_uri}/{ROUTE_ACCOUNT_REGISTER}");
    let log_in_uri = format!("{base_uri}/{ROUTE_ACCOUNT_LOG_IN}");

    let user_email = random_email();

    let response = client
        .post(&register_uri)
        .json(&json! ({
            "display_name": USER_ACCOUNT_TEST_DISPLAY_NAME,
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .post(&log_in_uri)
        .json(&json!({
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

/// Tests that when an email address already associated with an account is used to sign up, that a
/// CONFLICT response is returned.
#[tokio::test]
async fn test_email_in_use() {
    let server = TestServer::start().await;
    let client = reqwest::Client::new();

    let base_uri = server.base_uri();
    let register_uri = format!("{base_uri}/{ROUTE_ACCOUNT_REGISTER}");

    let user_email = random_email();

    // Register the email for the firt time and assert success
    let response = client
        .post(&register_uri)
        .json(&json! ({
            "display_name": USER_ACCOUNT_TEST_DISPLAY_NAME,
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Register the email for the second time and assert failure
    let response = client
        .post(&register_uri)
        .json(&json! ({
            "display_name": USER_ACCOUNT_TEST_DISPLAY_NAME,
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

/// Tests that when an email address that isn't associated with any account is used to log in, that
/// an UNAUTHORIZED response is returned.
#[tokio::test]
async fn test_unknown_email_log_in() {
    let server = TestServer::start().await;
    let client = reqwest::Client::new();

    let base_uri = server.base_uri();
    let log_in_uri = format!("{base_uri}/{ROUTE_ACCOUNT_LOG_IN}");

    let user_email = random_email();

    let response = client
        .post(&log_in_uri)
        .json(&json!({
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Tests that when the wrong password is used for an attempt to access an account, that an
/// UNAUTHORIZED response is returned.
#[tokio::test]
async fn test_wrong_password_log_in() {
    let server = TestServer::start().await;
    let client = reqwest::Client::new();

    let base_uri = server.base_uri();
    let register_uri = format!("{base_uri}/{ROUTE_ACCOUNT_REGISTER}");
    let log_in_uri = format!("{base_uri}/{ROUTE_ACCOUNT_LOG_IN}");

    let user_email = random_email();

    let response = client
        .post(&register_uri)
        .json(&json! ({
            "display_name": USER_ACCOUNT_TEST_DISPLAY_NAME,
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_VALID
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .post(&log_in_uri)
        .json(&json!({
            "email_address": &user_email,
            "password": USER_ACCOUNT_TEST_PASS_INVALID,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
