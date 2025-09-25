use axum::Json;
use serde::Serialize;


#[derive(Serialize)]
pub struct Data{
    message: String, 
    count: i32, 
    username: String

}

pub async fn get_json() -> Json<Data>{
    let data = Data {
        message: "quy".to_owned(), 
        count: 123, 
        username: "quy".to_owned()
    }; 
    Json(data)
}