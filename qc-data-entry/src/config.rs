use std::{env, path::PathBuf};

use config::{Config, File};
use serde::{Deserialize, Serialize};

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

// pub fn load_config(appname: &str) -> (Config, PathBuf) {
pub fn load_config(appname: &str) -> Config {
    let config_file_name = &format!("config_{}.toml", appname);

    // config in local folder overrides
    let mut local_config_path = PathBuf::from(r".");
    local_config_path.set_file_name(config_file_name);
    let mut config_path = home_dir();

    Config::builder()
        .set_default("config_path", local_config_path.display().to_string())
        .unwrap()
        .add_source(File::from(local_config_path))
        .build()
        .unwrap_or_else(|_| {
            config_path.extend([".config", appname, config_file_name]);
            Config::builder()
                .set_default("config_path", config_path.display().to_string())
                .unwrap()
                .add_source(File::from(config_path))
                .build()
                .unwrap_or_default()
        })
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
#[serde(default)]
pub struct DataEntryConfig {
    #[serde(skip_serializing)]
    config_path: PathBuf,

    #[serde(rename(deserialize = "blendsheet-path"))]
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
    // #[serde(deserialize_with = "deserialize_size_from_str")]
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
            log_file: default_log_path(),
            retain_file_name: default_retain_file_name(),
            retain_worksheet_name: "Sheet1".into(),
        }
    }
}

impl DataEntryConfig {
    pub fn load() -> Self {
        load_config("qc_data_entry")
            .try_deserialize::<Self>()
            .unwrap()
    }
}
