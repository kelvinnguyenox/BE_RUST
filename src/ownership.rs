// use axum::{
//     routing::{get, post},
//     response::{IntoResponse, Response}, 
//     http::StatusCode, 
//     Json, 
//     Router
// };

// #[tokio::main]
// async fn main() {
//     let addr = "127.0.0.1:3000"; 

//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     println!("Listening on {}", addr);
//     axum::serve(listener, router()).await.unwrap(); 
// }

// fn router() -> Router {
//     Router::new()
//         .route("/", get(hello_world).post(post_handler))
// }

// async fn hello_world() -> &'static str {
//     "Hello, World!"
// }

// async fn post_handler() -> impl IntoResponse {
//     (StatusCode::CREATED, Json(User {

//     }))
// }

// struct User {
//     name: String, 
//     age: u8
// }


fn main() {
    let x = 2; 
    let z; 
    {
        let y = 5; 
        z = life_time_exampl1(&x, &y); 
    }
    println!("z: {}", z); 
}

// fn example_1(x: i32) -> &i32 {
//     x
// }

// fn example_2<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
//     &x
// }
fn life_time_exampl1<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    &x
}