use std::{fs::OpenOptions, io::Write};

use super::Storage;

pub struct FileStorage {}

impl Storage for FileStorage {
  fn store_bookmark(
    &self,
    environment: &crate::environment::Environment,
    bookmark: &crate::bookmark::Bookmark,
  ) -> () {
    let path = environment.operating_system.ensure_storage_path();
    let mut file = OpenOptions::new().append(true).open(&path).unwrap();

    file.write_all(&bookmark.uuid.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
  }
}
