use reqwest;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
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
    pub address_2_uai: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Establishment {
    pub datasetid: String,
    pub recordid: String,
    pub fields: Fields,
    pub geometry: Option<Geometry>,
    pub record_timestamp: String,
}

pub async fn fetch_data() -> Result<Vec<Establishment>> {
    let url = "https://www.data.gouv.fr/fr/datasets/r/52be3f9b-bf98-4bc1-8a9d-a41f8cd82540";

    println!("Fetching data...");
    let data = reqwest::get(url)
        .await?
        .json::<Vec<Establishment>>()
        .await?;

    println!("data = {:?}", data);

    Ok(data)
}
