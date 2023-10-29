use anyhow::Result;
use hyper::StatusCode;
use rand::Rng;
use serde_json::{json, Value};

pub fn generate_user_login_payload() -> serde_json::Value {
    let mut rng = rand::thread_rng();
    let number: usize = rng.gen();
    json!({ "email": format!("email{}@domain.com",number), "password": "password", "name" : "name"})
}

pub fn generate_user_registration_payload() -> serde_json::Value {
    let mut rng = rand::thread_rng();
    let number: usize = rng.gen();
    json!({ "email": format!("email{}@domain.com",number), "password": "password", "name" : "name"})
}

#[tokio::test]
async fn double_registration() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user = generate_user_registration_payload();
    let user_clone = user.clone();
    let response = hc.do_post("/api/v1/authorization/register", user).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let response = hc
        .do_post("/api/v1/authorization/register", user_clone)
        .await?;
    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}

#[tokio::test]
async fn invalid_registration_payload() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let mut user_no_email = generate_user_registration_payload();
    user_no_email["email"] = json!("");
    let mut user_no_password = generate_user_registration_payload();
    user_no_password["password"] = json!("");
    let response = hc
        .do_post("/api/v1/authorization/register", user_no_email)
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let response = hc
        .do_post("/api/v1/authorization/register", user_no_password)
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}

#[tokio::test]
async fn on_invalid_login_return_bad_request() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_register = generate_user_registration_payload();
    let mut user_login = generate_user_login_payload();
    let response = hc
        .do_post("/api/v1/authorization/register", user_register)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    user_login["password"] = Value::String("".to_string());
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}

#[tokio::test]
async fn on_valid_login_return_ok_and_token() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_register = generate_user_registration_payload();
    let user_login =
        json!({"email": user_register["email"] , "password": user_register["password"]});

    let response = hc
        .do_post("/api/v1/authorization/register", user_register)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    let json = response.json_body().unwrap();
    let token = json["token"].as_str().unwrap();
    let status = json["status"].as_str().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(!token.is_empty());
    assert_eq!(status, "success");

    let response = hc.do_get("/api/v1/authorization/logout").await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}
