use std::path::PathBuf;

pub mod linux;

pub trait OperatingSystem {
  fn get_selection(&self) -> Option<String>;
  fn get_clipboard(&self) -> Option<String>;
  fn write_clipboard(&self, input: &str) -> ();
  fn ensure_storage_path(&self) -> PathBuf;

  fn prompt_text(&self, title: &str) -> String;
  fn interactive_select(&self, title: &str, options: &Vec<String>) -> String;
}
