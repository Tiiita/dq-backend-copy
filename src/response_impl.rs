use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, Json};
use serde::Serialize;


pub struct JsonResponse<T>(pub StatusCode, pub Json<T>);

impl<T: Serialize> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> Response<Body> {
        let (status, json) = (self.0, self.1);
        (status, json).into_response()
    }
}