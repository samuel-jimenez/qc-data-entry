use std::fmt;

use serde::Deserialize;
use serde_rusqlite::from_rows;

use crate::DB;

#[derive(Clone, Deserialize, Debug, Default)]
pub struct ProductCustomerName {
    product_customer_id: u32,
    product_name_customer: String,
}

impl fmt::Display for ProductCustomerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.product_name_customer.fmt(f)
    }
}

impl ProductCustomerName {
    pub fn select_info(db: &DB, product_id: &u32) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "
select product_customer_id, product_name_customer
		from bs.product_customer_line
		where product_id = ?
",
            )
            .unwrap();
        from_rows::<Self>(statement.query([product_id]).unwrap())
            .map(|x| x.unwrap())
            .collect()
    }
}
