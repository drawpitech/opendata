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

    /// Cache mode
    #[arg(short, long, env, default_value_t = true)]
    cache: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = sql::Database::new(&args.database).await?;

    if !args.cache || !db.is_db_populated().await? {
        println!("Refreshing cache.");
        db.insert_establishments(fetch_data::fetch_data().await?)
            .await?;
    } else {
        println!("Using cache.");
    }

    api::start(&args, db).await?;

    Ok(())
}
