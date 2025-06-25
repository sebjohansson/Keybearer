use Keybearer::Routing;

#[tokio::main]
async fn main() {
     Routing::serve()
        .await;
}