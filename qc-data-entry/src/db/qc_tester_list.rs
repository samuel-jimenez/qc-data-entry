use std::fmt;

use serde::Deserialize;
use serde_rusqlite::from_rows;

// use serde::{Deserialize, Serialize};
// use crate::errors::Result;
use crate::DB;

#[derive(Deserialize, Debug, Default)]
pub struct QcTesterList {
    qc_tester_id: u32,
    qc_tester_name: String,
}

impl fmt::Display for QcTesterList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.qc_tester_name.fmt(f)
    }
}

impl QcTesterList {
    pub fn select_qc_tester_all(db: &DB) -> Vec<Self> {
        let mut statement = db
            .prepare(
                "
	select qc_tester_id, qc_tester_name
		from bs.qc_tester_list
	where qc_tester_active = 1
	order by qc_tester_name
",
            )
            .unwrap();
        from_rows::<Self>(statement.query([]).unwrap())
            .map(|x| x.unwrap())
            .collect()
    }
}
