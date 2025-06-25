use axum::Router;
use axum::routing::{delete, get, post, put};

pub struct KeyController;

impl KeyController {
    pub fn app() -> Router {
        Router::new()
            .route("/", post(Self::create_token))
            .route("/{id}", get(Self::get_token))
            .route("/{id}", put(Self::update_token))
            .route("/{id}", delete(Self::delete_token))
    }

    pub async fn create_token(){

    }

    pub async fn get_token(){

    }

    pub async fn update_token(){

    }

    pub async fn delete_token(){
        
    }
}