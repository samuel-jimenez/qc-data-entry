use std::fmt;

use serde::Deserialize;
use serde_rusqlite::from_rows;

// use serde::{Deserialize, Serialize};
// use crate::errors::Result;
use crate::DB;

#[derive(Deserialize, Debug, Default)]
pub struct LotList {
    lot_id: u32,
    lot_name: String,
}

impl fmt::Display for LotList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lot_name.fmt(f)
        // write!(
        //     f,
        //     "{}",
        //     self.lot_name,
        // )
    }
}

impl LotList {
    pub fn select_lot_list_all(db: &DB) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "
    select
    lot_id, lot_name
    from bs.lot_list
",
            )
            .unwrap();
        from_rows::<Self>(statement.query([]).unwrap())
            .map(|x| x.unwrap())
            .collect()
    }
    pub fn select_lot_list_name(db: &DB, lot_name: &str) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "
    select
    lot_id, lot_name
    from bs.lot_list
    where  lot_name like ?

",
            )
            .unwrap();
        from_rows::<Self>(statement.query([lot_name]).unwrap())
            .map(|x| x.unwrap())
            .collect()
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
