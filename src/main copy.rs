// use axum::{
//     routing::{get, post},
//     response::IntoResponse,
//     http::StatusCode,
//     Json,
//     Router,
//     extract::{Query, Path}
// };
// use serde::{Serialize, Deserialize};
// use tokio::time;
// use std::collections::HashMap; 
// use std::fs::File; 
// use std::io::ErrorKind; 
// pub use self::error::{Error, Result};

// mod error;
// mod web;

// #[tokio::main]
// async fn main() {
//     let addr = "127.0.0.1:3000";

//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     println!("Listening on {}", addr);
//     axum::serve(listener, router()).await.unwrap();
// }

// fn router() -> Router {
//     Router::new()
//         .route("/hello", get(hello_world))
//         .route("/", get(hello_world_1))
//         // .route("/", get(hello_world).post(post_handler))
// }

// #[derive(Debug, Deserialize)]
// struct HelloParams {
//     name: Option<String>,
// }
// // async fn hello_world() -> &'static str {
// async fn hello_world(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     let name  = params.name.ok_or("Missing value");
//     println!("Received request with name: {:?}", name);
//     let user = User { name: "Alice".to_string(), age: 30 };
//     (StatusCode::CREATED, Json(user))
// }

// // async fn hello_world() -> &'static str {
// async fn hello_world_1() -> impl IntoResponse {
//     let user = User { name: "Alice".to_string(), age: 30 };
//     (StatusCode::CREATED, Json(user))
// }

// async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
//     // let user = User { name: name, age: 30 };
//     // (StatusCode::CREATED, Json(user))
// }
// // async fn post_handler() -> impl IntoResponse {
// //     let user = User { name: "Alice".to_string(), age: 30 };
// //     (StatusCode::CREATED, Json(user))
// // }

// #[derive(Serialize)]
// // struct User {
// //     name: String,
// //     age: u8
// // }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// impl User {
//     fn sayName(&self) -> String {
//         self.username.clone()
//     }

//     fn sayEmail(&self) -> String {
//         self.email.clone()
//     }
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn vector_rust() {
//     let mut v: Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);

//     let third: Option<&i32> = v.get(10);
//     match third {
//         Some(third) => println!("Value is {third}"),
//         None => println!("There is no third element"),
//     }
// }

// fn string_String() {
//     let mut s = String::from("foo");
//     s.push_str("bar");

//     s.push_str("b");

//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let mut s3 = String::new(); 
//     s3.push_str(&s1);
//     s3.push_str(&s2);
//     println!("The s3 value:  {}", s3);
//     println!("The s3 value:  {}", s1);
// }

// fn hash_map() {
//     let mut scores = HashMap::new(); 
    
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Red"), 20);

//     let team_name = String::from("Blue"); 
//     let score = scores.get(&team_name).unwrap();
//     let score1 = scores.get(&team_name).unwrap();

//     println!("The team scores: {}", score);
//     println!("The team scores: {}", score1);
// }

// fn handle_error() {
//     let greeting_file_result = File::open("hello.txt"); 
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file, 
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc, 
//                 Err(e) => panic!("Problem createing the file: {:?}",e),
//             }, 
//             other_error  =>{
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }
// #[tokio::main]
// async fn main() {
//     // let user = User {
//     //     active: true,
//     //     username: String::from("hoaitx"),
//     //     email: String::from("tonghoai.tnn@gmail.com"),
//     //     sign_in_count: 1,
//     // };
//     // println!("user say name {}", user.sayName());
//     println!("sfsfa");
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];
//     *num += 1;
//     println!("Third element is {}", *num);
//     println!("Vector is now {:?}", v);
//     let res = value_in_cents(Coin::Penny);
//     println!("Coin value: {}", res);
//     vector_rust();
//     string_String();
//     hash_map();
//     handle_error();
// }
