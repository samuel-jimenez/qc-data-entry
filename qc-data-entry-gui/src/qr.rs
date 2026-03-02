use serde::Deserialize;

#[derive(Deserialize)]
pub struct QRJson {
    #[serde(alias = "product_name")]
    product_type: String,
    lot_number: String,
}
