use axum::{body::StreamBody, extract::Path, response::IntoResponse, routing::get, Router};
use include_dir::{include_dir, Dir};
use tokio_util::io::ReaderStream;

use crate::{errors::Error, AppState};

static ASSETS: Dir<'_> = include_dir!("assets");

pub fn router() -> Router<AppState> {
    return Router::new().route("/*path", get(serve_assets));
}

async fn serve_assets(Path(path): Path<String>) -> Result<impl IntoResponse, Error> {
    let file = ASSETS
        .get_file(path)
        .ok_or(Error::NotFound(String::from("Not found")))?;
    let stream = ReaderStream::new(file.contents());
    let body = StreamBody::new(stream);
    return Ok(body);
}
