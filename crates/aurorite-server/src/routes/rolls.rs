use crate::requests::RollQuery;
use crate::responses::RollResult;
use crate::responses::{AuroriteErrorResponse, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::formulas::Dice;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;

async fn get_roll(
    State(_state): State<AuroriteState>,
    Query(query): Query<RollQuery>,
) -> FailableResponse<RollResult> {
    let dice = match (query.amount, query.max, query.line) {
        (Some(amount), Some(max), None) => Dice::new(amount, max, query.bonus),
        (None, None, Some(line)) => line.parse::<Dice>().map_err(|err| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                AuroriteErrorResponse::new(err).json(),
            )
        })?,
        _ => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                AuroriteErrorResponse::new("you must either provide (amount and max) or line")
                    .json(),
            ));
        }
    };
    Ok((
        StatusCode::OK,
        RollResult {
            parsed_amount: dice.amount,
            parsed_dice: dice.max,
            parsed_bonus: dice.bonus,
            result: dice.roll(),
        }
        .json(),
    ))
}

pub fn build_roll_routes() -> Router<AuroriteState> {
    Router::new().route("/", get(get_roll))
}
