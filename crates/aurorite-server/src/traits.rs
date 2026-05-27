use axum::Json;
use serde::Serialize;

pub trait IntoJson: Serialize + Sized {
    fn json(self) -> Json<Self> {
        Json(self)
    }
}

impl<T> IntoJson for T where T: Serialize + Sized {}
