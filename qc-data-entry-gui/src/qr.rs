use serde::Deserialize;

#[derive(Deserialize)]
pub struct QRJson {
    #[serde(alias = "product_name")]
    pub product_type: String,
    pub lot_number: String,
}
