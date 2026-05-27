use crate::build_app;
use crate::responses::{ClientInfo, ClientToken};
use axum::Json;
use axum::body::Body;
use axum::http::{Request, Response, StatusCode, header};
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_nonexisting_auth() {
    dotenvy::dotenv().ok();
    let app = build_app().await;

    let request = Request::post("/client/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({ "password": "notexists", "login": "notexists" })).unwrap(),
        ))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_existing_auth() {
    dotenvy::dotenv().ok();

    let mut app = build_app().await.into_service();

    let request = Request::post("/client/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({ "password": "aurorite", "login": "aurorite" })).unwrap(),
        ))
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res = serde_json::from_slice::<ClientToken>(&body).unwrap();

    let request = Request::get("/client/me")
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}", res.access_token),
        )
        .body(Body::empty())
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res = serde_json::from_slice::<ClientInfo>(&body).unwrap();
    assert_eq!(res.username, "aurorite");
    assert_eq!(res.display_name, None);
}
