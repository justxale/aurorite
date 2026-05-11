use serde::Serialize;
use axum::Json;

pub trait IntoJson: Serialize + Sized {
    fn json(self) -> Json<Self> {
        Json(self)
    }
}

impl<T> IntoJson for T where T: Serialize + Sized {}