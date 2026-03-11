use std::fmt;

use serde::Deserialize;
use serde_rusqlite::from_rows;

// use serde::{Deserialize, Serialize};
// use crate::errors::Result;
use crate::{ProductCustomerName, ProductLot, SamplePoint, DB};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ProductLine {
    pub(super) product_id: u32,
    product_moniker_name: String,
    product_name_internal: String,
}

impl fmt::Display for ProductLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.product_moniker_name, self.product_name_internal,
        )
    }
}

impl ProductLine {
    pub fn select_info_all(db: &DB) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "select product_id, product_name_internal, product_moniker_name
            from bs.product_line
            join bs.product_moniker using (product_moniker_id)
            order by product_moniker_name, product_name_internal",
            )
            .unwrap();
        from_rows::<Self>(statement.query([]).unwrap())
            .map(|x| x.unwrap())
            .collect()
    }
    pub fn select_product_lots(&self, db: &DB) -> Vec<ProductLot> {
        ProductLot::select_info(db, &self.product_id)
    }
    pub fn select_customer_names(&self, db: &DB) -> Vec<ProductCustomerName> {
        ProductCustomerName::select_info(db, &self.product_id)
    }
    pub fn select_sample_points(&self, db: &DB) -> Vec<SamplePoint> {
        SamplePoint::select_info(db, &self.product_id)
    }
}
/*

type BaseProduct struct {
    Product_name             string `json:"product_name"`
    Lot_number               string `json:"lot_number"`
    Sample_point             string
    Tester                   nullable.NullString `json:"Tester"`
    Visual                   bool
    Product_id               int64
    Lot_id                   int64
    Product_Lot_id           int64
    Product_name_customer_id nullable.NullInt64
    Product_name_customer    string `json:"customer_product_name"`
    Blend                    *blender.ProductBlend
    Valid                    bool
}

let row1 = Example { id: 1, name: "first name".into() };
connection.execute("INSERT INTO example (id, name) VALUES (:id, :name)", to_params_named(&row1).unwrap().to_slice().as_slice()).unwrap();
// and limiting the set of fields that are to be serialized*/
