use crate::{
  interpreter::{text::TextInterpreter, Interpreter},
  os::{linux::Linux, OperatingSystem},
  storage::{fs::FileStorage, Storage},
};

pub struct Environment {
  pub interpreters: Vec<Box<dyn Interpreter>>,
  pub operating_system: Box<dyn OperatingSystem>,
  pub storage: Box<dyn Storage>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      interpreters: vec![Box::new(TextInterpreter {})],
      operating_system: Box::new(Linux {}),
      storage: Box::new(FileStorage {}),
    }
  }
}
