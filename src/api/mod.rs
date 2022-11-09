use crate::bookmark::Bookmark;

pub mod cli;

pub trait Api {
  fn prompt_text(&self, title: &str) -> String;
  fn select_bookmark<'a>(&self, bookmarks: &'a Vec<Bookmark>) -> &'a Bookmark;
}
