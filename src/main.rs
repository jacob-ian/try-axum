use axum_try::{
    assets,
    users::{self, User},
    AppState,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let users: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
    let state = AppState { users };

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/users", users::router())
        .nest("/assets", assets::router())
        .with_state(state);

    println!("Listening on 0.0.0.0:4000");
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> Html<&'static str> {
    return Html("<h1>Hello world</h1><img src=\"/assets/test.png\"/>");
}
