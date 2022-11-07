use environment::Environment;

mod bookmark;
mod cli;
mod environment;
mod interpreter;
mod os;
mod storage;

fn main() {
  let environment = Environment::new();

  let clipboard = environment.operating_system.get_clipboard();

  println!("{:?}", clipboard);
}
