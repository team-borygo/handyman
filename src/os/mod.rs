pub mod linux;

pub trait OperatingSystem {
  fn get_clipboard(&self) -> Option<String>;
}
