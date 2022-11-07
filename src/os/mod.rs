use std::path::PathBuf;

pub mod linux;

pub trait OperatingSystem {
  fn get_clipboard(&self) -> Option<String>;
  fn ensure_storage_path(&self) -> PathBuf;
}
