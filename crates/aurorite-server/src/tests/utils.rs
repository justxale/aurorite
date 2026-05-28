use tokio_util::bytes::Bytes;
use http_body_util::BodyExt;
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::routing::RouterIntoService;
use serde::Deserialize;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};
use crate::responses::{ClientToken, FullCampaignInfo};

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

pub async fn post_request(
    app: &mut RouterIntoService<Body>,
    uri: &str,
    body: &Value,
    authorization: Option<&String>
) -> (StatusCode, Bytes) {
    let mut request = Request::post(uri).header(header::CONTENT_TYPE, "application/json");
    if let Some(token) = authorization {
        request = request.header(
            header::AUTHORIZATION, format!("Bearer {}", token)
        )
    }
    let request = request.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    let status = response.status();
    (status, response.into_body().collect().await.unwrap().to_bytes())
}

pub async fn get_request(
    app: &mut RouterIntoService<Body>,
    uri: &str,
    authorization: Option<&String>
) -> (StatusCode, Bytes) {
    let mut request = Request::get(uri);
    if let Some(token) = authorization {
        request = request.header(
            header::AUTHORIZATION, format!("Bearer {}", token)
        )
    }
    let request = request.body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    let status = response.status();
    (status, response.into_body().collect().await.unwrap().to_bytes())
}

pub async fn delete_request(
    app: &mut RouterIntoService<Body>,
    uri: &str,
    authorization: Option<&String>
) -> StatusCode {
    let mut request = Request::delete(uri);
    if let Some(token) = authorization {
        request = request.header(
            header::AUTHORIZATION, format!("Bearer {}", token)
        )
    }
    let request = request.body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    response.status()
}