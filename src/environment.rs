use crate::{
  interpreter::{text::TextInterpreter, Interpreter},
  os::{linux::Linux, OperatingSystem},
};

pub struct Environment {
  pub interpreters: Vec<Box<dyn Interpreter>>,
  pub operating_system: Box<dyn OperatingSystem>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      interpreters: vec![Box::new(TextInterpreter {})],
      operating_system: Box::new(Linux {}),
    }
  }
}
