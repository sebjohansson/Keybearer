mod api;

use axum:: {Router};

pub struct Routing;

impl Routing {
    pub async fn serve() {
        let app = Router::new()
            .nest("/api/v1", api::app());

        let listener = tokio::net::TcpListener::bind("").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

}