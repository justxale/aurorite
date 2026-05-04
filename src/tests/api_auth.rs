use http_body_util::BodyExt;
use axum::body::Body;
use axum::http::{Request, header, StatusCode, Response};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};
use crate::build_app;
use crate::responses::ClientToken;

#[tokio::test]
async fn test_not_existing_auth() {
    dotenvy::dotenv().ok();
    let app = build_app().await;

    let request = Request::post("/client/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({ "password": "notexists", "login": "notexists" })).unwrap()
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
            serde_json::to_vec(&json!({ "password": "aurorite", "login": "aurorite" })).unwrap()
        ))
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await.unwrap()
        .call(request)
        .await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let _res = serde_json::from_slice::<ClientToken>(&body).unwrap();
}

