use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode>{
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn return_201() -> Response {
    (
        StatusCode::CREATED, "This is a 2002".to_owned()
    ).into_response()
} 