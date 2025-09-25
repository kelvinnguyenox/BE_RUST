use axum::{async_trait, body::HttpBody, extract::{FromRequest, RequestParts}, http::StatusCode, BoxError, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Must be a valid email"))]
    pub username: String,

    #[validate(length(min = 8, message = "Must have at least 8 characters"))] 
    pub password: String,
}

#[async_trait]
impl<B> FromRequest <B> for RequestUser 
where  
 B: HttpBody + Send, 
 B::Data:Send, 
 B::Error: Into<BoxError>
{
    type Rejection  = (StatusCode, String); 
        
    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        dbg!("befor");
        let Json(user) = request
            .extract::<Json<RequestUser>>()
            .await
            .map_err(|error| (
            (StatusCode::BAD_REQUEST, format!("{}", error))))?;
        
        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST,format!("{}", errors)));
        }
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Json<RequestUser>{
    dbg!(123);
    dbg!(&user);

    Json(user)
}