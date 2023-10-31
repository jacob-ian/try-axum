use axum::{extract::State, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{errors::Error, AppState};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Clone, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

pub fn router() -> Router<AppState> {
    return Router::new().route("/", get(get_users).post(create_user));
}

async fn get_users(State(state): State<AppState>) -> (StatusCode, Json<Vec<User>>) {
    let users = state.users.lock().await.to_owned();
    return (StatusCode::OK, Json(users));
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, Error> {
    let mut users = state.users.lock().await;
    let mut exists = false;
    for user in users.to_vec() {
        if user.username == payload.username {
            exists = true;
            break;
        }
    }

    if exists {
        return Err(Error::BadRequest(String::from("username exists")));
    }

    let user = User {
        id: format!("{}-id", payload.username),
        username: payload.username,
    };

    users.push(user.clone());
    return Ok(Json(user));
}
