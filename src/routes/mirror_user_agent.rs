
use axum::{
    extract::{TypedHeader},
    headers::UserAgent,
    http::{Request, header::HeaderMap},
};

pub async fn mirror_user_agent(headers: HeaderMap) -> String {
    let message_value = headers.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_owned(); 
    message
}