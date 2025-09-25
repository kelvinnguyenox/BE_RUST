use axum::{
    Extension, 
    http::{Request, StatusCode}, 
    middleware::Next, 
    response::Response
}; 
use crate::routes::SharedData;

#[derive(Clone)]
pub struct HeaderMessage(pub String); 

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String{
    shared_data.message
}

pub async fn read_middleware_message_header(Extension(message): Extension<HeaderMessage>) -> String{
    message.0
}

pub async fn set_middleware_message_header<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = request.headers(); 
    let message = headers
        .get("message")
        .ok_or_else(
            || StatusCode::BAD_REQUEST)?;
    let message = message.to_str().map_err(|_error| {
        StatusCode::BAD_REQUEST
    })?.to_owned(); 

    request.extensions_mut().insert(HeaderMessage(message)); 
    Ok(next.run(request).await)
}