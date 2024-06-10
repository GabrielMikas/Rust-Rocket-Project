use models::{Card, NewCard};
use rocket_testing::*;
use axum::{ routing::get, Router};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/yg", get(|| async {"Should work"}));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}