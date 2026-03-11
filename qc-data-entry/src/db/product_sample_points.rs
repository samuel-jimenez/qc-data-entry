use std::fmt;

use serde::Deserialize;
use serde_rusqlite::from_rows;

use crate::DB;

#[derive(Deserialize, Debug, Default)]
pub struct SamplePoint {
    sample_point_id: u32,
    sample_point: String,
}

impl fmt::Display for SamplePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.sample_point.fmt(f)
    }
}

impl SamplePoint {
    pub fn select_info(db: &DB, product_id: &u32) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "
select distinct sample_point_id, sample_point
		from bs.product_lot
		join bs.qc_samples using (lot_id)
		join bs.product_sample_points using (sample_point_id)
	where product_id = ?
	order by sample_point_id
",
            )
            .unwrap();
        from_rows::<Self>(statement.query([product_id]).unwrap())
            .map(|x| x.unwrap())
            .collect()
    }
}
