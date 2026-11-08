use crate::requests::RollQuery;
use crate::responses::RollResult;
use crate::responses::{AuroriteErrorResponse, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::formulas::Dice;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    params(
        ("amount" = Option<u16>, Query, description = "Number of dice"),
        ("max" = Option<u16>, Query, description = "Maximum value on each die"),
        ("bonus" = Option<i16>, Query, description = "Roll bonus"),
        ("line" = Option<String>, Query, description = "Dice expression")
    ),
    tag = "rolls",
    responses((status = OK, description = "Dice roll result"), (status = UNPROCESSABLE_ENTITY, body = AuroriteErrorResponse))
)]
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

pub fn build_roll_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new().routes(routes!(get_roll))
}
