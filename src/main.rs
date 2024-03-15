use axum::{routing::get, Router};
use clap::Parser;

/// Palachias backend server
#[derive(Parser, Debug)]
struct Args {
    /// Port to listen on
    #[arg(short, long, env, default_value_t = 5050)]
    port: u16,

    /// Address to listen on
    #[arg(short, long, env, default_value = "0.0.0.0")]
    address: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind((args.address, args.port)).await?;

    println!("Listening on http://{:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
