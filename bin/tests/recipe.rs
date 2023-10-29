use httpc_test::Response;
use hyper::StatusCode;
use rand::Rng;
use serde_json::json;

pub fn default_login_user() -> serde_json::Value {
    json!({ "email": "email-test@domain.com", "password": "password"})
}

pub fn validate_login(response: Response) {
    let json = response.json_body().unwrap();
    let token = json["token"].as_str().unwrap();
    let status = json["status"].as_str().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!token.is_empty());
    assert_eq!(status, "success");
}

pub fn malformed_recipe() -> serde_json::Value {
    json!({ "name": "", "image": "image.url", "method": "a method"})
}

pub fn wellformed_recipe() -> serde_json::Value {
    let mut rng = rand::thread_rng();
    json!({ "name": format!("name{}",rng.gen::<usize>()), "image":  format!("image.url{}",rng.gen::<usize>()  ), "method": format!("a method{}",rng.gen::<usize>()  )})
}

#[tokio::test]
async fn on_nameless_recipe_return_bad_request() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let recipe = malformed_recipe();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc.do_post("/api/v1/recipes", recipe).await?;
    let json = response.json_body().unwrap();
    let expected_json = json!({
      "error": "name is required"
    });
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(json, expected_json);
    Ok(())
}

#[tokio::test]
async fn on_wellformed_recipe_return_success() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let recipe = wellformed_recipe();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc.do_post("/api/v1/recipes", recipe).await?;
    let json = response.json_body().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(!json["uuid"].as_str().unwrap().is_empty());
    Ok(())
}

#[tokio::test]
async fn on_empty_name_update_recipe_return_bad_request() -> Result<(), Box<dyn std::error::Error>>
{
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let mut recipe = wellformed_recipe();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc.do_post("/api/v1/recipes", recipe.clone()).await?;
    let json = response.json_body().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let uuid = json["uuid"].as_str().unwrap();
    assert!(!json["uuid"].as_str().unwrap().is_empty());
    recipe["name"] = json!("");
    let response = hc
        .do_put(&format!("/api/v1/recipes/{}", uuid), recipe)
        .await?;
    let json = response.json_body().unwrap();
    let expected_json = json!({
      "error": "name is required"
    });
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(json, expected_json);
    Ok(())
}

#[tokio::test]
async fn on_invalid_uuid_update_recipe_return_not_found() -> Result<(), Box<dyn std::error::Error>>
{
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let recipe = wellformed_recipe();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc
        .do_put(&format!("/api/v1/recipes/{}", uuid::Uuid::new_v4()), recipe)
        .await?;
    let json = response.json_body().unwrap();
    let expected_json = json!({
      "error": "recipe not found"
    });
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(json, expected_json);
    Ok(())
}

#[tokio::test]
async fn on_update_recipe_return_success() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc.do_post("/api/v1/recipes", wellformed_recipe()).await?;
    let json = response.json_body().unwrap();
    let uuid = json["uuid"].as_str().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(!uuid.is_empty());
    let response = hc
        .do_put(&format!("/api/v1/recipes/{}", uuid), wellformed_recipe())
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn on_delete_recipe_return_success() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc.do_post("/api/v1/recipes", wellformed_recipe()).await?;
    let json = response.json_body().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let uuid = json["uuid"].as_str().unwrap();
    assert!(!json["uuid"].as_str().unwrap().is_empty());
    let response = hc.do_delete(&format!("/api/v1/recipes/{}", uuid)).await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn on_invalid_uuid_delete_recipe_return_not_found() -> Result<(), Box<dyn std::error::Error>>
{
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc
        .do_delete(&format!("/api/v1/recipes/{}", uuid::Uuid::new_v4()))
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn on_get_recipe_return_success() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let mut expected_json = wellformed_recipe();
    let response = hc.do_post("/api/v1/recipes", expected_json.clone()).await?;
    let json = response.json_body().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let uuid = json["uuid"].as_str().unwrap();
    assert!(!json["uuid"].as_str().unwrap().is_empty());
    let response = hc.do_get(&format!("/api/v1/recipes/{}", uuid)).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let json = response.json_body().unwrap();
    expected_json["uuid"] = json!(uuid);
    assert_eq!(expected_json, json);
    Ok(())
}

#[tokio::test]
async fn on_invalid_uuid_get_recipe_return_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;
    let user_login = default_login_user();
    let response = hc
        .do_post("/api/v1/authorization/login", user_login)
        .await?;
    validate_login(response);
    let response = hc
        .do_get(&format!("/api/v1/recipes/{}", uuid::Uuid::new_v4()))
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
