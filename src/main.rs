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

  match clipboard {
    Some(clipboard) => {
      println!("Clipboard content: {:?}", clipboard);

      let interpreter = environment
        .interpreters
        .iter()
        .find(|i| i.check(&clipboard));

      match interpreter {
        Some(interpreter) => {
          let bookmark = interpreter.interpet(&clipboard);

          environment.storage.store_bookmark(&environment, &bookmark);
        }
        None => {
          println!("No matching interpreter found for \"{}\"", clipboard);
        }
      }
    }
    None => {}
  }
}
