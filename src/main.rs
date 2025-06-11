use axum::{extract::Path, routing::{delete, get, post}, Json, Router};

struct Token {
    id: i32,
    token: String
}

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/tokens/new", post(create_token))
    .route("/tokens/:id/get", get(get_token))
    .route("/tokens/:id/rotate", post(rotate_token))
    .route("/tokens/:id/delete", delete(delete_token));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_token() -> Json<Token> {
    Json(Token { id: 1, token: "dummy-token".to_string() })
}

async fn get_token(Path(id): Path<i32>) -> Json<String> {
    format!("Retrieved token with id: {}", id).into()
}

async fn rotate_token(Path(id): Path<i32>) -> Json<String> {
    format!("Rotated token with id: {}", id).into()
}

async fn delete_token(Path(id): Path<i32>) -> Json<String> {
    format!("Deleted token with id: {}", id).into()
}