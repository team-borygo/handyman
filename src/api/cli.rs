use super::Api;

pub struct Cli;
use dialoguer::Input;

impl Api for Cli {
  fn prompt_text(&self, title: &str) -> String {
    Input::new().with_prompt(title).interact_text().unwrap()
  }
}
