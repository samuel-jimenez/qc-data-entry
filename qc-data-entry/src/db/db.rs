use std::path::PathBuf;

use log::info;
use rusqlite::{Connection, Params, Rows, Statement};
use semver::Version;
use thiserror::Error;

// use anyhow::Result;
use crate::errors::Result;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no rows found")]
    NoRows,
    #[error(
        "database version mismatch: required: {required_db_version}, found: {found_db_version}"
    )]
    VersionMismatch {
        required_db_version: Version,
        found_db_version: Version,
    },
    // currently does nothing. maybe if we coalece all errors in this mod into this type.
    // #[error(transparent)]
    // DatabaseError(#[from] rusqlite::Error),
    // #[error(transparent)]
}

// #[derive(Debug, Default)]
#[derive(Debug)]
pub struct DB {
    connection: Connection,
}
impl Default for DB {
    fn default() -> Self {
        Self {
            connection: Connection::open_in_memory().unwrap(),
        }
    }
}

impl DB {
    /// Open a new connection to an in-memory SQLite database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite open call or version check fails.
    pub fn new(db_file: PathBuf) -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        connection
            .prepare("attach ? as 'bs'")?
            .execute((db_file.to_string_lossy(),))?;

        let db = Self { connection };
        db.check()?;
        // log.Println("Info: Using db:", config.DB_FILE)
        info!("Using db: {}", db_file.display());

        // qc.DBinit(qc_db)
        Ok(db)
    }

    pub fn check(&self) -> Result<()> {
        fn new_version(major: u8, minor: u8, revision: u8) -> Version {
            Version::new(major.into(), minor.into(), revision.into())
        }

        let mut version_statement = self.connection.prepare(
            "select database_version_major, database_version_minor, database_version_revision
		from bs.database_info",
        )?;

        let db_version = version_statement
            .query_map([], |row| {
                Ok(new_version(row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .next()
            .ok_or(Error::NoRows)??;

        let program_version = Version::parse("0.0.4")?;

        if db_version != program_version {
            Err(Error::VersionMismatch {
                required_db_version: program_version,
                found_db_version: db_version,
            })?;
        }

        Ok(())
    }

    // Result<Statement<'_>>
    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        Ok(self.connection.prepare(sql)?)
    }

    pub fn query<'a, P>(&self, statement: &'a mut Statement<'a>, params: P) -> Result<Rows<'a>>
    where
        P: Params,
    {
        Ok(statement.query(params)?)
    }

    // // Deserialize
    // pub fn select_product_info_all(&self) -> Result<Vec<ProductLine>> {
    //     let mut statement = self.connection.prepare(
    //         "select product_id, product_name_internal, product_moniker_name
    //         from bs.product_line
    //         join bs.product_moniker using (product_moniker_id)
    //         order by product_moniker_name, product_name_internal",
    //     )?;
    //
    //     Ok(statement
    //         .query([])?
    //         .mapped(|row| {
    //             Ok(format!(
    //                 "{} {}",
    //                 row.get::<_, String>("product_moniker_name")?,
    //                 row.get::<_, String>(2)?
    //             ))
    //         })
    //         .map(|x| x.unwrap())
    //         .collect())
    // }

    //

    // let db_version = statement
    //     .query_map([], |row| {
    //         Ok(new_version(row.get(0)?, row.get(1)?, row.get(2)?))
    //     })?
    //     .next()
    //     .ok_or(Error::NoRows)??;
    //
    // let program_version = Version::parse("0.0.4")?;
    //
    // if db_version != program_version {
    //     Err(Error::VersionMismatch {
    //         required_db_version: program_version,
    //         found_db_version: db_version,
    //     })?;
    // }
    //
    // Ok(())
    //  }

    //     fn misc(db: &Connection) -> Result<Statement> {
    //     let name = "Lisa";
    //     let age = 8;
    //     let smart = true;
    //     Ok(prepare_and_bind!(db, "SELECT $name, @age, :smart;"))
    // }
}
// impl Drop for DB {
//     fn drop(&mut self) {
//     }
// }
