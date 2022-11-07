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

  clipboard
    .and_then(|clipboard| {
      println!("Clipboard content: {:?}", clipboard);

      environment
        .interpreters
        .iter()
        .find(|i| i.check(&clipboard))
        .map(|interpreter| (clipboard, interpreter))
    })
    .and_then(|(clipboard, interpreter)| {
      let bookmark = interpreter.interpet(&clipboard);

      environment.storage.store_bookmark(&environment, &bookmark);

      Some(())
    });
}
