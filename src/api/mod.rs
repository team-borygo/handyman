pub mod cli;

pub trait Api {
  fn prompt_text(&self, title: &str) -> String;
}
