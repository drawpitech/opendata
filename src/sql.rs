use anyhow::{anyhow, Result};
use serde::Serialize;
use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};

use super::fetch_data::JsonEstablishment;

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

#[derive(FromRow, Serialize, Debug)]
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

impl TryFrom<JsonEstablishment> for Establishment {
    type Error = anyhow::Error;

    fn try_from(esta: JsonEstablishment) -> Result<Self, Self::Error> {
        let Some([latitude, longitude]) = esta.fields.geores.as_deref() else {
            return Err(anyhow!("Missing georeference data"));
        };

        let Some(address) = esta.fields.adresse_2_ua else {
            return Err(anyhow!("Missing address"));
        };

        let Some(city) = esta.fields.libelle_commune else {
            return Err(anyhow!("Missing city"));
        };

        let Some(postal_code) = esta.fields.code_postal else {
            return Err(anyhow!("Missing postal code"));
        };

        Ok(Establishment {
            record_id: esta.recordid,
            kind: esta
                .fields
                .app_libelle_activite_etablissement
                .unwrap_or_default(),
            name: esta.fields.app_libelle_etablissement.unwrap_or_default(),
            siret: esta.fields.siret.unwrap_or_default(),
            inspection_date: esta.record_timestamp,
            evaluation: esta.fields.synthese_eval_sanit.unwrap_or_default(),
            latitude: *latitude,
            longitude: *longitude,
            postal_code,
            address,
            city,
        })
    }
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
                record_id TEXT NOT NULL PRIMARY KEY,
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
            ) STRICT;
            CREATE INDEX IF NOT EXISTS establishments_latitude_index ON establishments (latitude);
            CREATE INDEX IF NOT EXISTS establishments_longitude_index ON establishments (longitude);",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_establishments(&self, raw_data: Vec<JsonEstablishment>) -> Result<()> {
        let establishments: Vec<Establishment> = raw_data
            .into_iter()
            .filter_map(|e| e.try_into().ok())
            .collect();

        println!("Inserting data");

        let mut tx = self.pool.begin().await?;
        for e in establishments {
            sqlx::query(
            "INSERT OR REPLACE INTO establishments (record_id, kind, name, siret, address, city, postal_code, latitude, longitude, inspection_date, evaluation)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            )
            .bind(e.record_id)
            .bind(e.kind)
            .bind(e.name)
            .bind(e.siret)
            .bind(e.address)
            .bind(e.city)
            .bind(e.postal_code)
            .bind(e.latitude)
            .bind(e.longitude)
            .bind(e.inspection_date)
            .bind(e.evaluation)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        println!("Data inserted");

        Ok(())
    }

    pub async fn is_db_populated(&self) -> Result<bool> {
        let count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM (SELECT 0 FROM establishments LIMIT 1);")
                .fetch_one(&self.pool)
                .await?;
        Ok(count != 0)
    }

    pub async fn list_establishments_bounds(&self) -> Result<Vec<Establishment>> {
        todo!("list")
    }
}
