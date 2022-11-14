use serde_json::{from_str, to_string};
use std::{
  fs::{File, OpenOptions},
  io::{BufRead, BufReader, Write},
};

use crate::{bookmark::Bookmark, environment::Environment};

use super::Storage;

pub struct FileStorage {}

impl Storage for FileStorage {
  fn store_bookmark(&self, environment: &Environment, bookmark: &Bookmark) -> () {
    let path = environment.operating_system.ensure_storage_path();
    let mut file = OpenOptions::new().append(true).open(&path).unwrap();

    let serialized = to_string(&bookmark).unwrap();
    file.write_all(&serialized.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
  }

  fn get_bookmarks(&self, environment: &Environment) -> Box<dyn Iterator<Item = Bookmark>> {
    let path = environment.operating_system.ensure_storage_path();
    let file = File::open(path).unwrap();

    let iter = BufReader::new(file)
      .lines()
      .map(|line| line.unwrap())
      .filter(|line| !line.trim().is_empty())
      .map(|line| {
        let bookmark: Bookmark = from_str(&line).unwrap();
        bookmark
      });

    Box::new(iter)
  }

  fn clear(&self, environment: &Environment) -> () {
    let path = environment.operating_system.ensure_storage_path();
    File::create(&path).unwrap();
  }
}
