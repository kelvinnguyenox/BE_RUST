use axum::{Json};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct RequestUser {
    username: Option<String>, 
    password: String, 
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) -> Json<RequestUser> {
    dbg!(&user); 
    Json(user)
}  