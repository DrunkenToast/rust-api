extern crate dotenv;

mod model;
mod handler;  
mod service;

use std::{env, net::SocketAddr, sync::Arc};

use service::{arduino::{Arduino, ArduinoState}, sql::open_database_connection, scheduler::start_scheduler};
use axum::{handler::Handler, Extension};
use dotenv::dotenv;
use tokio::{signal, sync::Mutex};

use axum::Router;
use tower_http::trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let serial_port = env::var("SERIAL_PORT").expect("SERIAL_PORT not defined in .env");
    let arduino: ArduinoState = Arc::new(Mutex::new(Arduino::new(serial_port).await));
    let db = Arc::new(Mutex::new(open_database_connection().expect("Database failed to open")));

    start_scheduler(arduino.clone(), db.clone());
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust-api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest("/health", handler::health::routes()) // Routes
        .nest("/dht", handler::dht::routes())
        .nest("/msg", handler::msg::routes())
        .nest("/led", handler::led::routes())
        .fallback(handler::handler_404.into_service()) // Fallback
        .layer( // Onions
            TraceLayer::new_for_http()
            .on_request(
                DefaultOnRequest::new().level(Level::DEBUG)
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
            )
        )
        .layer(Extension(arduino)) // Extensions
        .layer(Extension(db));

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
