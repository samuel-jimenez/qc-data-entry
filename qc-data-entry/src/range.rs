use serde::Deserialize;
use serde_rusqlite::from_row;

use crate::DB;

// #[derive(Default, Clone)]
// #[derive(Default, Clone, Copy)]
// #[derive(Debug, Default, Clone, Copy)]
// #[derive(Debug, Default, Deserialize, Clone, Copy)]
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Range {
    pub min: Option<f32>,
    pub target: Option<f32>,
    pub max: Option<f32>,
    pub method: Option<String>,
    //   	    nullable.NullString
    // Valid     bool
    measure: bool,
    publish: bool,
    // precision: int,//TODO allow specify precision
}
// {:.*}",   5, 0.01);
impl Range {
    // pub fn check(&self, val: f32) -> bool {
    //     self.min.is_none_or(|x| x <= val) && self.max.is_none_or(|x| x >= val)
    // }
    pub fn valid(&self) -> bool {
        self.measure
            || self.publish
            || self.min.is_some()
            || self.target.is_some()
            || self.max.is_some()
    }

    pub fn check_min(&self, val: f32) -> bool {
        self.min.is_none_or(|x| x <= val)
    }
    pub fn check_max(&self, val: f32) -> bool {
        self.max.is_none_or(|x| x >= val)
    }
    pub fn map(&self, val_map: fn(f32) -> f32) -> Self {
        Self {
            min: self.min.map(val_map),
            target: self.target.map(val_map),
            max: self.max.map(val_map),
            method: self.method.clone(),
            measure: self.measure,
            publish: self.publish,
        }
        // self.set_impl(val.clone().map(|x| map(x)).unwrap_or_default())
    }
}

impl Range {
    pub fn select_product_lot_product(
        db: &DB,
        product_id: &u32,
        qc_test_type_id: &u32,
    ) -> Option<Self> {
        let mut statement = db
            .prepare(
                "
               	select
                qc_test_method_name as method,
	 val_measure as measure,
	 val_publish as publish,
	 val_min as min,
	 val_target as target,
	 val_max as max

	from bs.product_ranges_measured
	left join bs.qc_test_methods using (qc_test_method_id)
	where product_id = ?1
	and qc_test_type_id = ?2
        ",
            )
            .unwrap();

        statement
            .query_one_optional([product_id, qc_test_type_id], from_row::<Self>)
            .unwrap()
    }
}
// todo!()
impl From<Vec<Option<f32>>> for Range {
    fn from(val_in: Vec<Option<f32>>) -> Self {
        Self {
            min: val_in[0],
            target: val_in[1],
            max: val_in[2],
            method: None,
            measure: false,
            publish: false,
        }
    }
}
