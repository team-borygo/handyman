use crate::{
  interpreter::{get_interpreters, Interpreter},
  os::{linux::Linux, OperatingSystem},
};

pub struct Environment {
  pub interpreters: Vec<Box<dyn Interpreter>>,
  pub operating_system: Box<dyn OperatingSystem>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      interpreters: get_interpreters(),
      operating_system: Box::new(Linux {}),
    }
  }
}
