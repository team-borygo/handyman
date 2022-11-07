use crate::{bookmark::Bookmark, environment::Environment};

pub mod fs;

pub trait Storage {
  fn store_bookmark(&self, environment: &Environment, bookmark: &Bookmark) -> ();
}
