use axum_try::{
    errors::Error,
    users::{self, User},
    AppState,
};
use include_dir::{include_dir, Dir};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::io::ReaderStream;

use axum::{
    body::StreamBody,
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let users: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
    let state = AppState { users };

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/users", users::router())
        .route("/assets/*path", get(serve_assets))
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

static ASSETS: Dir<'_> = include_dir!("assets");
async fn serve_assets(Path(path): Path<String>) -> Result<impl IntoResponse, Error> {
    let file = ASSETS
        .get_file(path)
        .ok_or(Error::NotFound(String::from("Not found")))?;
    let stream = ReaderStream::new(file.contents());
    let body = StreamBody::new(stream);
    return Ok(body);
}
