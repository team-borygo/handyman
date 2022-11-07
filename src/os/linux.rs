use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;
use std::process::Command;
use std::str;

use super::OperatingSystem;

pub struct Linux {}

impl OperatingSystem for Linux {
  fn get_clipboard(&self) -> Option<String> {
    let output = Command::new("xsel")
      .arg("-o")
      .arg("-b")
      .output()
      .expect("failed to execute /xsel/");

    if output.stdout.len() == 0 {
      None
    } else {
      Some(
        str::from_utf8(&output.stdout)
          .expect("failed to parse clipboard content into UTF-8")
          .to_string(),
      )
    }
  }

  fn ensure_storage_path(&self) -> std::path::PathBuf {
    let home = env::var("HOME").expect("HOME env variable is not set");
    let default_xdg_data = PathBuf::from(home).join("/.local/share");
    let xdg_data = env::var("XDG_DATA_HOME")
      .and_then(|p| Ok(PathBuf::from(p)))
      .unwrap_or(default_xdg_data);

    let directory = xdg_data.join("handyman");

    create_dir_all(&directory).expect("Cannot create directory for file storage");

    let path = directory.join("storage");

    if !path.exists() {
      File::create(&path).expect("Cannot create file for file storage");
    }

    path
  }
}
