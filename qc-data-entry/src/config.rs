use std::{env, fs, io::Write, path::PathBuf};

use anyhow::Result;
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct DataEntryConfig {
    #[serde(skip_serializing)]
    config_path: PathBuf,

    // #[serde(rename(deserialize = "blendsheet-path"))]
    #[serde(alias = "blendsheet-path")]
    pub blendsheet_path: PathBuf,
    pub coa_filepath: PathBuf,
    pub coa_template_path: PathBuf,
    pub db_path: PathBuf,
    pub font_size: u64,
    pub label_path: PathBuf,
    pub log_file: PathBuf,
    #[serde(alias = "retain_file_name")]
    pub retain_file_name: PathBuf,
    pub retain_worksheet_name: Box<str>,
}
impl Default for DataEntryConfig {
    fn default() -> Self {
        Self {
            config_path: PathBuf::from(r"."),
            blendsheet_path: default_blendsheet_path(),
            coa_filepath: default_coa_filepath(),
            coa_template_path: default_coa_template_path(),
            db_path: default_db_path(),
            font_size: 15,
            label_path: default_label_path(),
            log_file: default_log_path().join("qc_data_entry.log"),
            retain_file_name: default_retain_file_name(),
            retain_worksheet_name: "Sheet1".into(),
        }
    }
}

impl DataEntryConfig {
    pub fn load_from(config_path: Option<PathBuf>) -> Self {
        load_config("qc_data_entry", config_path)
            .try_deserialize::<Self>()
            .unwrap()
    }

    pub fn load() -> Self {
        DataEntryConfig::load_from(None)
    }

    pub fn save(&self) -> Result<()> {
        use fs::File;

        let mut file = File::create(self.config_path.clone())?;
        match &*self.config_path.extension().unwrap().display().to_string() {
            "toml" => {
                file.write_all(toml::to_string_pretty(self)?.as_bytes())?;
            }
            _ => (),
        }
        Ok(())
    }

    pub fn config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    pub fn db_file(&self) -> PathBuf {
        self.db_path.join("qc.sqlite3")
    }
}

fn home_dir() -> PathBuf {
    env::home_dir().unwrap_or(PathBuf::from(r"."))
}

fn base_path() -> PathBuf {
    home_dir().join("Isomeric Industries Incorporated/ISOMERIC-BIG SPRING - Documents")
}

fn gen_path() -> PathBuf {
    base_path().join("gen")
}

fn default_db_path() -> PathBuf {
    base_path().join("QUALITY/log")
}

fn default_log_path() -> PathBuf {
    base_path().join("var/log")
}

fn default_blendsheet_path() -> PathBuf {
    base_path().join(
        "PRODUCTION/BLEND SHEETS - ACTIVE TO COMPLETED/C.13 BLEND SHEETS PENDING BLEND DETAILS",
    )
}

fn default_coa_filepath() -> PathBuf {
    base_path().join("QUALITY/COA-Filled")
}

fn default_coa_template_path() -> PathBuf {
    base_path().join("QUALITY/COA TEMPLATES")
}

fn default_label_path() -> PathBuf {
    gen_path().join("labels")
}
fn default_retain_file_name() -> PathBuf {
    PathBuf::from(r"RETAIN-SAMPLE-TRACKING.xlsx")
}

pub fn load_config(appname: &str, supplied_config_path: Option<PathBuf>) -> Config {
    let config_file_name = &format!("config_{}.toml", appname);
    let config_paths = match supplied_config_path {
        Some(config_path) => &mut vec![config_path.join(config_file_name)],
        None => {
            // // config in local folder overrides
            let mut local_config_path = PathBuf::from(r".");
            local_config_path.set_file_name(config_file_name);

            let mut config_path = home_dir();
            config_path.extend([".config", appname, config_file_name]);
            &mut vec![config_path, local_config_path] // pop() processes in reverse order
        }
    };
    load_config_loop(PathBuf::new(), config_paths)
}

fn load_config_loop(config_path: PathBuf, config_paths: &mut Vec<PathBuf>) -> Config {
    use config::File;
    if config_paths.len() == 0 {
        Config::builder()
            .set_default("config_path", config_path.display().to_string())
            .unwrap()
            .build()
            .unwrap_or_default()
    } else {
        let config_path = config_paths.pop().unwrap();
        Config::builder()
            .set_default("config_path", config_path.display().to_string())
            .unwrap()
            .add_source(File::from(config_path.clone()))
            .build()
            .unwrap_or_else(|_| load_config_loop(config_path, config_paths))
    }
}
