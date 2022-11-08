use crate::{bookmark::Bookmark, environment::Environment};

pub mod fs;
pub mod text;
pub mod url;

pub trait Interpreter {
  fn check(&self, input: &str) -> bool;
  fn interpet(&self, environment: &Environment, input: &str) -> Bookmark;
  fn belongs(&self, interpreted_by: &str) -> bool;
}
