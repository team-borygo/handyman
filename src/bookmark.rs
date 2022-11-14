use std::fmt::Display;

use serde::{Deserialize, Serialize};
use unicode_truncate::UnicodeTruncateStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
  pub uuid: String,
  pub title: String,
  pub content: BookmarkContent,
  pub interpreted_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookmarkContent {
  content: String,
}

impl Bookmark {
  pub fn new(title: String, content: String, interpreted_by: String) -> Bookmark {
    let uuid = Uuid::new_v4().to_string();

    Bookmark {
      uuid,
      title,
      content: BookmarkContent::new(content),
      interpreted_by,
    }
  }
}

impl Display for Bookmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.title)
  }
}

impl BookmarkContent {
  fn new(content: String) -> BookmarkContent {
    BookmarkContent { content }
  }

  pub fn short(&self) -> String {
    self.content.unicode_truncate(30).0.replace('\n', "")
  }

  pub fn long(&self) -> &str {
    &self.content
  }
}

impl Display for BookmarkContent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.content)
  }
}
