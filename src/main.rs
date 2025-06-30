use keybearer::Routing;

#[tokio::main]
async fn main() {
     Routing::serve()
        .await;
}