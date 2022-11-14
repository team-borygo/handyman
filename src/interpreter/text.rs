use crate::{bookmark::Bookmark, environment::Environment};
use unicode_truncate::UnicodeTruncateStr;

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
    let prompt_title = format!("Bookmark title for ({}...)", &input.unicode_truncate(30).0);

    Bookmark::new(
      environment.api.prompt_text(&prompt_title),
      input.to_string(),
      self.get_name(),
    )
  }

  fn belongs(&self, interpreted_by: &str) -> bool {
    interpreted_by == self.get_name()
  }
}
