mod handler;

use std::net::SocketAddr;

use axum::{Router};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/health", handler::health::routes())
        .nest("/temp", handler::temp::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
