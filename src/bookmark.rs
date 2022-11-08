use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
  pub uuid: String,
  pub title: String,
  pub content: String,
  pub interpreted_by: String,
}

impl Bookmark {
  pub fn new(title: String, content: String, interpreted_by: String) -> Bookmark {
    let uuid = Uuid::new_v4().to_string();

    Bookmark {
      uuid,
      title,
      content,
      interpreted_by,
    }
  }
}
