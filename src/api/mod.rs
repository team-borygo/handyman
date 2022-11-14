use crate::{bookmark::Bookmark, command::Program};

pub mod cli;

pub trait Api {
  fn start(&self) -> Program;
  fn prompt_text(&self, title: &str) -> String;
  fn select_bookmark<'a>(&self, bookmarks: &'a Vec<Bookmark>) -> &'a Bookmark;
  fn list_bookmarks(&self, iter: Box<dyn Iterator<Item = Bookmark>>) -> ();
  fn confirm(&self, title: &str) -> bool;
}
