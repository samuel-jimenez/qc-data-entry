// use crate::DB;

use crate::{ProductCustomerName, ProductLine, ProductLot, QcTesterList, SamplePoint};

// #[derive(Deserialize, Debug, Default, Clone)]
#[derive(Clone)]
pub struct SampleInfo {
    pub product_name: ProductLine,
    pub lot_name: ProductLot,
    pub tester_name: QcTesterList,
    pub customer_name: Option<ProductCustomerName>,
    pub sample_name: Option<SamplePoint>,
}
// impl SampleInfo {
//     pub fn clone(&self) -> _ {
//         todo!()
//     }
// }

// // check, sav, print,
// // coa, xl bs
// impl SampleInfo {
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
