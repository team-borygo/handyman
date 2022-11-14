use crate::{bookmark::Bookmark, environment::Environment};

pub mod fs;

pub trait Storage {
  fn store_bookmark(&self, environment: &Environment, bookmark: &Bookmark) -> ();

  fn get_bookmarks(&self, environment: &Environment) -> Box<dyn Iterator<Item = Bookmark>>;

  fn clear(&self, environment: &Environment) -> ();
}
