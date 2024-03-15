use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonFields {
    pub ods_type_etablissement: Option<String>,
    pub app_libelle_activite_etablissement: Option<String>,
    pub libelle_commune: Option<String>,
    pub siret: Option<String>,
    pub app_libelle_etablissement: Option<String>,
    pub numero_inspection: Option<String>,
    pub geores: Option<Vec<f64>>,
    pub filtre: Option<String>,
    pub date_inspection: Option<String>,
    pub code_postal: Option<String>,
    pub synthese_eval_sanit: Option<String>,
    pub adresse_2_ua: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGeometry {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEstablishment {
    pub datasetid: String,
    pub recordid: String,
    pub fields: JsonFields,
    pub geometry: Option<JsonGeometry>,
    pub record_timestamp: String,
}

pub async fn fetch_data() -> Result<Vec<JsonEstablishment>> {
    let url = "https://www.data.gouv.fr/fr/datasets/r/52be3f9b-bf98-4bc1-8a9d-a41f8cd82540";

    println!("Fetching data...");
    let data = reqwest::get(url)
        .await?
        .json::<Vec<JsonEstablishment>>()
        .await?;

    Ok(data)
}
