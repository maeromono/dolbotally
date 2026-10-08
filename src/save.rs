use std::{fs::{File, create_dir_all, read_to_string, rename}, io::{Error, ErrorKind, Result, Write}, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

use crate::dt::Counter;

#[derive(Serialize, Deserialize)]
pub struct DolbotallyCounters {
    pub counters: Vec<Counter>,
    pub counter_index: usize,
}

impl Default for DolbotallyCounters {
    fn default() -> Self {
        Self {
            counters: vec![],
            counter_index: 0
        }
    }
}

fn save_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("page.codeberg", "enmkdev", "dolbotally")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "cannot locate data directory"))?;
    Ok(dirs.data_dir().join("counters.toml"))
}

pub fn save_counters(counters: &DolbotallyCounters) -> Result<()> {
    let path = save_path()?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(counters).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let tmp_path = path.with_extension("tmp");
    let mut tmp = File::create(&tmp_path)?;
    tmp.write_all(contents.as_bytes())?;
    tmp.sync_all()?;
    rename(&tmp_path, &path)?;
    Ok(())
}

pub fn load_counters() -> Result<DolbotallyCounters> {
    let path = save_path()?;
    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(DolbotallyCounters::default()),
        Err(err) => return Err(err),
    };
    match from_str(&contents) {
        Ok(counters) => Ok(counters),
        Err(err) => {
            let _ = rename(&path, &path.with_extension("corrupt"));
            Err(Error::new(ErrorKind::InvalidData, err))
        }
    }
}
