use axum::http::StatusCode;
use crate::build_app;
use crate::responses::RollResult;
use crate::tests::utils::get_request;

#[tokio::test]
async fn test_invalid_rolls() {
    dotenvy::dotenv().ok();

    let mut app = build_app().await.into_service();
    let (status, _) = get_request(&mut app, "/rolls?line=1n29", None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = get_request(&mut app, "/rolls?line=1dk+2", None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = get_request(&mut app, "/rolls?amount=1&max=20&bonus=20&line=1d20", None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = get_request(&mut app, "/rolls", None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_valid_rolls() {
    dotenvy::dotenv().ok();
    let mut app = build_app().await.into_service();

    let (status, body) = get_request(&mut app, "/rolls?line=1d10", None).await;
    assert_eq!(status, StatusCode::OK);
    let body = serde_json::from_slice::<RollResult>(&body).unwrap();
    assert_eq!(body.parsed_amount, 1);
    assert_eq!(body.parsed_dice, 10);
    assert_eq!(body.parsed_bonus, None);

    let (status, body) = get_request(&mut app, "/rolls?line=3d20-2", None).await;
    assert_eq!(status, StatusCode::OK);
    let body = serde_json::from_slice::<RollResult>(&body).unwrap();
    assert_eq!(body.parsed_amount, 3);
    assert_eq!(body.parsed_dice, 20);
    assert_eq!(body.parsed_bonus, Some(-2));

    let (status, body) = get_request(&mut app, "/rolls?line=3d20%2B2", None).await;
    assert_eq!(status, StatusCode::OK);
    let body = serde_json::from_slice::<RollResult>(&body).unwrap();
    assert_eq!(body.parsed_amount, 3);
    assert_eq!(body.parsed_dice, 20);
    assert_eq!(body.parsed_bonus, Some(2));

    let (status, body) = get_request(&mut app, "/rolls?amount=3&max=4&bonus=2", None).await;
    assert_eq!(status, StatusCode::OK);
    let body = serde_json::from_slice::<RollResult>(&body).unwrap();
    assert_eq!(body.parsed_amount, 3);
    assert_eq!(body.parsed_dice, 4);
    assert_eq!(body.parsed_bonus, Some(2));

    let (status, body) = get_request(&mut app, "/rolls?amount=3&max=4&bonus=-2", None).await;
    assert_eq!(status, StatusCode::OK);
    let body = serde_json::from_slice::<RollResult>(&body).unwrap();
    assert_eq!(body.parsed_amount, 3);
    assert_eq!(body.parsed_dice, 4);
    assert_eq!(body.parsed_bonus, Some(-2));
}