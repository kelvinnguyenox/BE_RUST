use axum::{ extract::{Path, Query}, response::IntoResponse, Extension, Json };
use http::StatusCode;
use sea_orm::{ sqlx::database, DatabaseConnection, EntityTrait, QueryFilter };
use serde::{ Deserialize, Serialize };
use crate::database::tasks::{ self, Entity as Tasks, Model };

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    tittle: String,
    priority: Option<String>,
    description: Option<String>,
}
pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id).one(&database).await.unwrap();

    //    Ok(Json(ResponseTask {
    //         id: task.id,
    //         tittle: task.title,
    //         priority: task.priority,
    //         description: task.description
    //     }))

    if let Some(task) = task {
        Ok(
            Json(ResponseTask {
                id: task.id,
                tittle: task.title,
                priority: task.priority,
                description: task.description,
            })
        )
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_all_task(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let task = Tasks::find()
        .filter(tasks::Column::Priority::eq(query_params.priority))
        .all(&database).await
        .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR })?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            tittle: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(task))
}
