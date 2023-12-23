mod logging;
mod router;

#[tokio::main]
async fn main() {
    logging::init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router::create()).await.unwrap();
}
