use axum::{response::Redirect, routing::get, Router};use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
    .route("/hello_rust_people", get(hello_rust_people))
    .fallback(anything_else);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn anything_else() -> Redirect {
    Redirect::to("/hello_rust_people")
}

async fn hello_rust_people() -> &'static str {
    "Hello, rust people!"
}