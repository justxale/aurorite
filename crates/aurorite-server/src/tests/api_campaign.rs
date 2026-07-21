use crate::build_app;
use crate::responses::ClientCampaigns;
use crate::tests::utils::{auth_client, delete_request, get_request, post_request};
use aurorite_dataflow::dto::CampaignDto;
use aurorite_util::uuid::EncodedUuid;
use axum::http::StatusCode;

#[tokio::test]
#[tracing_test::traced_test]
async fn test_creating_campaign() {
    dotenvy::dotenv().ok();
    let (_, app) = build_app().await;
    let mut app = app.into_service();
    let token = auth_client(&mut app).await;
    let (status, body) = get_request(&mut app, "/campaigns", Some(&token.access_token)).await;
    let res = serde_json::from_slice::<ClientCampaigns>(&body).unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(res.campaigns.len(), 0);

    let (status, body) = post_request(
        &mut app,
        "/campaigns",
        &serde_json::json!({ "title": "Best tested DnD!" }),
        Some(&token.access_token),
    )
    .await;
    let res = serde_json::from_slice::<CampaignDto>(&body).unwrap();
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(res.title, "Best tested DnD!");

    let (status, body) = get_request(&mut app, "/campaigns", Some(&token.access_token)).await;
    let res = serde_json::from_slice::<ClientCampaigns>(&body).unwrap();
    println!("{:?}", body);
    assert_eq!(status, StatusCode::OK);
    assert_eq!(res.campaigns.len(), 1);

    let status = delete_request(
        &mut app,
        &format!("/campaigns/{}", EncodedUuid(res.campaigns[0].id)),
        Some(&token.access_token),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = get_request(&mut app, "/campaigns", Some(&token.access_token)).await;
    let res = serde_json::from_slice::<ClientCampaigns>(&body).unwrap();
    println!("{:?}", body);
    assert_eq!(status, StatusCode::OK);
    assert_eq!(res.campaigns.len(), 0);
}
