use crate::{bookmark::Bookmark, environment::Environment};

pub mod fs;
pub mod text;
pub mod url;

pub trait Interpreter {
  fn check(&self, input: &str) -> bool;
  fn interpet(&self, environment: &Environment, input: &str) -> Bookmark;
  fn belongs(&self, interpreted_by: &str) -> bool;
}

pub fn match_interpreter<'a>(
  environment: &'a Environment,
  input: &str,
) -> Option<&'a Box<dyn Interpreter>> {
  environment.interpreters.iter().find(|i| i.check(input))
}
