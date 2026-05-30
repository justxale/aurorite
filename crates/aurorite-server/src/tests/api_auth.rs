use super::utils::auth_client;
use crate::build_app;
use crate::responses::ClientInfo;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_nonexisting_auth() {
    dotenvy::dotenv().ok();
    let app = build_app().await;

    let request = Request::post("/client/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(
                &serde_json::json!({ "password": "notexists", "login": "notexists" }),
            )
            .unwrap(),
        ))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_existing_auth() {
    dotenvy::dotenv().ok();

    let mut app = build_app().await.into_service();
    let token = auth_client(&mut app).await;

    let request = Request::get("/client/me")
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}", token.access_token),
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
