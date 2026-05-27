use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use tower::{Service, ServiceExt};
use tracing_test::traced_test;
use crate::build_app;
use crate::responses::CampaignInfo;
use crate::tests::utils::auth_client;

#[tokio::test]
#[traced_test]
async fn test_creating_campaign() {
    dotenvy::dotenv().ok();
    let mut app = build_app().await.into_service();
    let token = auth_client(&mut app).await;
    let request = Request::post("/campaigns")
        .header(
            header::AUTHORIZATION, format!("Bearer {}", token.access_token)
        )
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&serde_json::json!({ "title": "Best tested DnD!" })).unwrap()
        ))
        .unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let status = response.status();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("{:?}", &body);
    let res = serde_json::from_slice::<CampaignInfo>(&body).unwrap();
    println!("{:?}", res);
    assert_eq!(status, StatusCode::OK);
    assert_eq!(res.title, "Best tested DnD!")
}