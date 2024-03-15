use std::env;

use anyhow::Context;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let port = match env::var("PORT") {
        Ok(port) => port.parse().context("invalid port")?,
        Err(_) => 5050,
    };

    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;

    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
