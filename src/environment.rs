use crate::{
  api::{cli::Cli, Api},
  interpreter::{text::TextInterpreter, url::UrlInterpreter, Interpreter},
  os::{linux::Linux, OperatingSystem},
  storage::{fs::FileStorage, Storage},
};

pub struct Environment {
  pub interpreters: Vec<Box<dyn Interpreter>>,
  pub operating_system: Box<dyn OperatingSystem>,
  pub storage: Box<dyn Storage>,
  pub api: Box<dyn Api>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      interpreters: vec![Box::new(UrlInterpreter {}), Box::new(TextInterpreter {})],
      operating_system: Box::new(Linux {}),
      storage: Box::new(FileStorage {}),
      api: Box::new(Cli {}),
    }
  }
}
