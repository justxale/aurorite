use http_body_util::BodyExt;
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::routing::RouterIntoService;
use serde_json::json;
use tower::{Service, ServiceExt};
use crate::responses::ClientToken;

pub async fn auth_client(app: &mut RouterIntoService<Body>) -> ClientToken {
    let request = Request::post("/client/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({ "password": "aurorite", "login": "aurorite" })).unwrap(),
        ))
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice::<ClientToken>(&body).unwrap()
}