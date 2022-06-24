extern crate dotenv;

mod handler;
mod arduino;
mod model;
use std::{env, net::SocketAddr, sync::Arc};

use arduino::Arduino;
use axum::{handler::Handler, Extension};
use dotenv::dotenv;
use tokio::{signal, sync::Mutex};

use axum::{Router};
use tower::ServiceBuilder;
use tower_http::{trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse, DefaultMakeSpan}, LatencyUnit};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let serial_port = env::var("SERIAL_PORT").expect("Serial port not defined in .env");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust-api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest("/health", handler::health::routes())
        .nest("/temp", handler::temp::routes())
        .nest("/led", handler::led::routes())
        .fallback(handler::handler_404.into_service())
        .layer(
            TraceLayer::new_for_http()
            .on_request(
                DefaultOnRequest::new().level(Level::DEBUG)
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
            )
        )
        .layer(Extension(Arc::new(Mutex::new(Arduino::new(serial_port).await))));

    let addr: SocketAddr = env::var("ADDR").unwrap()
        .parse().expect("Cannot parse server address");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };


    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received, starting graceful shutdown");
}
