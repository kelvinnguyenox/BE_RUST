mod hello_world; 
mod mirror_body_json; 
mod mirror_body_string; 
mod path_variables; 
mod query_params; 
mod mirror_user_agent;
mod middleware_message; 
mod always_errors; 
mod get_json; 
mod validate_data; 
mod custom_json_extractor; 
mod create_task; 
mod get_task; 

use axum::{routing::{get, delete, put, post}, Router, body::Body, middleware};
use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};
use http::Method; 
use axum::Extension; 

use hello_world::hello_world; 
use mirror_body_string::mirror_body_string; 
use mirror_body_json::mirror_body_json; 
use path_variables::path_variables; 
use query_params::query_params; 
use mirror_user_agent::mirror_user_agent; 
use always_errors::always_errors; 
use self::middleware_message::{middleware_message,read_middleware_message_header,set_middleware_message_header}; 
use get_json::get_json; 
use validate_data::validate_with_serde; 
use custom_json_extractor::custom_json_extractor; 
use create_task::create_task; 
use get_task::get_one_task; 
use get_task::get_all_task; 

#[derive(Clone)]
pub struct SharedData {
    pub message: String, 
}

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any); 
    let shared_data = SharedData {
        message: "Hello from shared data".to_owned()
    };

    Router::new()
        .route("/read_middelware_custom_header", get(read_middleware_message_header))
        .route_layer(middleware::from_fn(set_middleware_message_header))
        .route("/", post(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always_errors", get(always_errors))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/get_all_task", get(get_all_task))
        .route("/create_task", post(create_task))
        .route("/task/:task_id", get(get_one_task))
        .layer(Extension(database))
}