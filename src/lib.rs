use std::sync::Arc;

use tokio::sync::Mutex;
use users::User;

pub mod errors;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
}
