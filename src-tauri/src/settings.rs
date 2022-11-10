use crate::throw;
use atomicwrites::{AtomicFile, OverwriteBehavior};
use std::{fs::File, path::PathBuf};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
  pub repo: String,
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      repo: ".".to_string(),
    }
  }
}

impl Settings {
  fn settings_file() -> PathBuf {
    let conf_dir = tauri::api::path::config_dir().expect("No data dir");
    return conf_dir.join("gittite").join("settings.json");
  }

  pub fn load() -> Result<Self, String> {
    let mut settings_file = match File::open(Self::settings_file()){
      Ok(file) => file,
      Err(e) => throw!("Error opening file: {}", e.to_string()),
    };
    let mut json_str = String::new();
    match settings_file.read_to_string(&mut json_str) {
      Ok(_) => (),
      Err(e) => throw!("Error reading file: {}", e),
    };
    match serde_json::from_str(&mut json_str) {
      Ok(settings) => Ok(settings),
      Err(e) => throw!("Error parsing file: {}", e.to_string())
    }
  }

  pub fn save(&self) -> Result<(), String> {
    let mut json = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    match self.serialize(&mut ser) {
      Ok(_) => (),
      Err(e) => throw!("Error saving content: {}", e.to_string()),
    }
    match write_atomically(&Self::settings_file(), &json) {
      Ok(_) => (),
      Err(e) => throw!("Error saving: {}", e.to_string()),
    }
    Ok(())
  }
}

pub fn ensure_parent_exists(file_path: &PathBuf) -> Result<(), String> {
  if let Some(parent) = file_path.parent() {
    if let Err(e) = std::fs::create_dir_all(parent) {
      throw!("Error creating parent folder: {}", e.to_string());
    }
  }
  Ok(())
}

pub fn write_atomically(file_path: &PathBuf, buf: &[u8]) -> Result<(), String> {
  ensure_parent_exists(&file_path)?;
  let af = AtomicFile::new(&file_path, OverwriteBehavior::AllowOverwrite);
  match af.write(|f| f.write_all(&buf)) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}
