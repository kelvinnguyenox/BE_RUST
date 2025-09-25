use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};
use crate::database::tasks::{self}; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestTask {
    tittle: String, 
    priority: Option<String>, 
    description: Option<String>
}

pub async fn create_task(Extension(database): Extension<DatabaseConnection>, Json(request_task): Json<RequestTask>) {
    let new_task = tasks::ActiveModel{
        priority: Set(request_task.priority), 
        title: Set(request_task.tittle), 
        description: Set(request_task.description), 
        ..Default::default()
    }; 
    dbg!(&new_task);
    let result = new_task.save(&database).await.unwrap(); 
     
    dbg!(result); 
}