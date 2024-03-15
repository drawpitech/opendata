use axum::Router;
use axum::routing::{get, Route};
use super::{sql, Args};

pub async fn start(args: &Args, database: &sql::Database) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", get(|| async { "API is alive and running!" }))
        .route("/api/get_near", get(get_near))
        .route("/api/info", get(info));

    let listener = tokio::net::TcpListener::bind((&args.address, &args.port)).await?;

    println!("Listening on http://{:?}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}

async fn get_near() -> &'static str {
    todo!("get_near")
}

async fn info() -> &'static str {
    todo!("info")
}
