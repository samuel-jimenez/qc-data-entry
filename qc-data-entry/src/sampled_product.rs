use serde::Deserialize;

// use crate::DB;
use crate::{ProductCustomerName, ProductLine, ProductLot, QcTesterList, SampleInfo, SamplePoint};

// #[derive(Deserialize, Debug, Default, Clone)]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct SampledProduct {
    pub product_name: ProductLine,
    pub lot_name: ProductLot,
    pub tester_name: QcTesterList,
    pub customer_name: Option<ProductCustomerName>,
    pub sample_name: Option<SamplePoint>,
    pub visual: bool,
    pub ph: Option<f32>,
    pub sg: Option<f32>,
    pub density: Option<f32>,
    pub string_test: Option<u8>,
    pub viscosity: Option<u16>,
}
impl From<SampleInfo> for SampledProduct {
    fn from(value: SampleInfo) -> Self {
        Self {
            product_name: value.product_name,
            lot_name: value.lot_name,
            tester_name: value.tester_name,
            customer_name: value.customer_name,
            sample_name: value.sample_name,
            ..Default::default() // ph: todo!(),
                                 // sg: todo!(),
                                 // density: todo!(),
                                 // string_test: todo!(),
                                 // viscosity: todo!(),
        }
    }
}

impl SampledProduct {
    fn save(&self) {}
    fn store(&self) {}

    pub fn check_sample_single(&self) {}
    pub fn check_sample_double(samples: Vec<SampledProduct>) {}
}
// check, sav, print,
// coa, xl bs
// impl SampledProduct {
//     pub fn select_info_all(db: &DB) -> Vec<Self> {
//         let mut statement = db
//             .prepare(
//                 "select product_id, product_name_internal, product_moniker_name
//             from bs.product_line
//             join bs.product_moniker using (product_moniker_id)
//             order by product_moniker_name, product_name_internal",
//             )
//             .unwrap();
//         from_rows::<Self>(statement.query([]).unwrap())
//             .map(|x| x.unwrap())
//             .collect()
//     }
// }
