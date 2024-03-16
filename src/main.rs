mod api;
mod fetch_data;
mod sql;

use clap::Parser;

/// Palachias backend server
#[derive(Parser, Debug)]
struct Args {
    /// Port to listen on
    #[arg(short, long, env, default_value_t = 5050)]
    port: u16,

    /// Address to listen on
    #[arg(short, long, env, default_value = "0.0.0.0")]
    address: String,

    /// Path to the database file
    #[arg(long, env, default_value = "palachias.sqlite")]
    database: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = sql::Database::new(&args.database).await?;

    db.insert_establishments(fetch_data::fetch_data().await?)
        .await?;

    api::start(&args, &db).await?;

    Ok(())
}
