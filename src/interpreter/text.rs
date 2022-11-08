use crate::{bookmark::Bookmark, environment::Environment};

use super::Interpreter;

pub struct TextInterpreter {}

impl TextInterpreter {
  fn get_name(&self) -> String {
    "text".to_string()
  }
}

impl Interpreter for TextInterpreter {
  fn check(&self, _input: &str) -> bool {
    true
  }

  fn interpet(&self, environment: &Environment, input: &str) -> crate::bookmark::Bookmark {
    Bookmark::new(
      environment.api.prompt_text("Bookmark title"),
      input.to_string(),
      self.get_name(),
    )
  }

  fn belongs(&self, interpreted_by: &str) -> bool {
    interpreted_by == self.get_name()
  }
}
