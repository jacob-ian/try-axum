use axum_try::{
    errors::Error,
    users::{self, User},
    AppState,
};
use rust_embed::RustEmbed;
use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;

use axum::{extract::Path, response::Html, routing::get, Router};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

#[tokio::main]
async fn main() {
    let users: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
    let state = AppState { users };

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/users", users::router())
        .route("/static/:name", get(handle_static))
        .with_state(state);

    println!("Listening on 0.0.0.0:4000");
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> Html<&'static str> {
    return Html("<h1>Hello world</h1><img src=\"/static/test.png\"/>");
}

async fn handle_static(Path(name): Path<String>) -> Result<Cow<'static, [u8]>, Error> {
    // this should be a stream
    let file = Assets::get(&name).ok_or(Error::NotFound(String::from("Not found")))?;
    return Ok(file.data);
}
