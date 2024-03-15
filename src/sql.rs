use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Establishment {
    pub record_id: String,
    pub kind: String,
    pub name: String,
    pub siret: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub inspection_date: String,
    pub evaluation: String,
}

impl Database {
    pub async fn new(database: &str) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(database)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;

        let db = Self { pool };
        db.create_table().await?;

        Ok(db)
    }

    pub async fn get_establishment(&self, record_id: &str) -> Result<Option<Establishment>> {
        let establishment = sqlx::query_as("SELECT * FROM establishments WHERE record_id = $1")
            .bind(record_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(establishment)
    }

    async fn create_table(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS establishments (
                record_id TEXT NOT NULL CONSTRAINT establishments_pk PRIMARY KEY,
                kind      TEXT NOT NULL,
                name      TEXT NOT NULL,
                siret     TEXT NOT NULL,
                address   TEXT NOT NULL,
                city      TEXT NOT NULL,
                postal_code TEXT NOT NULL,
                latitude  REAL NOT NULL,
                longitude REAL NOT NULL,
                inspection_date TEXT NOT NULL,
                evaluation TEXT NOT NULL
            ) STRICT;",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
