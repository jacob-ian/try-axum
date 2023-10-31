use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
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
    return Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:user_id", get(get_user_by_id).delete(delete_user_by_id));
}

async fn get_users(State(state): State<AppState>) -> (StatusCode, Json<Vec<User>>) {
    let users = state.users.lock().await.to_owned();
    return (StatusCode::OK, Json(users));
}

async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, Error> {
    let users = state.users.lock().await.to_owned();
    let mut found: Option<User> = None;
    for user in users {
        if user.id == user_id {
            found = Some(user);
            break;
        }
    }
    if let Some(u) = found {
        return Ok(Json(u));
    } else {
        return Err(Error::NotFound(String::from("User not found")));
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), Error> {
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
    return Ok((StatusCode::CREATED, Json(user)));
}

async fn delete_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<(StatusCode, ()), Error> {
    let mut users = state.users.lock().await;
    let mut updated: Vec<User> = Vec::new();
    let mut found = false;
    for user in users.to_vec() {
        if user.id == user_id {
            found = true;
        }
        if user.id != user_id {
            updated.push(user);
        }
    }

    if !found {
        return Err(Error::NotFound(String::from("User not found")));
    }

    *users = updated;
    return Ok((StatusCode::NO_CONTENT, ()));
}
