use axum::{extract::Query, Json};
use serde::{Serialize, Deserialize}; 

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    message: String, 
    id: i32, 
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}