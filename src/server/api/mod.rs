mod key_controller;

use axum::routing::*;
use self::{key_controller::KeyController};

pub fn app() -> Router {
    Router::new()
        .nest("/key", KeyController::app())
}