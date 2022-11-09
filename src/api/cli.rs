use crate::bookmark::Bookmark;

use super::Api;

pub struct Cli;
use dialoguer::{console::Term, Input, Select};

impl Api for Cli {
  fn prompt_text(&self, title: &str) -> String {
    Input::new().with_prompt(title).interact_text().unwrap()
  }

  fn select_bookmark<'a>(&self, bookmarks: &'a Vec<Bookmark>) -> &'a Bookmark {
    let selection = Select::new()
      .items(&bookmarks)
      .default(0)
      .interact_on_opt(&Term::stderr())
      .unwrap()
      .unwrap();

    &bookmarks[selection]
  }
}
