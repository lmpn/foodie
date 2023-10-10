#![allow(unused)]

use anyhow::Result;

// #[tokio::test]
// async fn quick_dev() -> Result<()> {
//     let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
//     let data = r#"{
//         "name": "Kitty",
//         "image": "http://placeimg.com/640/480/food",
//         "method": ""
//     }"#;

//     let json: serde_json::Value = serde_json::from_str(&data)?;
//     hc.do_put("/api/v1/recipes", json).await?.print().await?;
//     Ok(())
// }

#[tokio::test]
async fn quick_dev() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"{
    "email": "lmpneto137@gmail.com",
    "password": "password"
}"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
    hc.do_post("/api/v1/authorization/login", json)
        .await?
        .print()
        .await?;

    let data = r#"{
        "name": "Kitty",
        "image": "http://placeimg.com/640/480/food",
        "method": "a method"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    hc.do_post("/api/v1/recipes", json).await?.print().await?;

    let data = r#"{
        "name": "Kitty",
        "image": "http://placeimg.com/640/480/food",
        "method": "a method"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    hc.do_post("/api/v1/recipes", json).await?.print().await?;

    Ok(())
}
