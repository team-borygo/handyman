use std::process;

use command::Command;
use environment::Environment;
use interpreter::match_interpreter;

mod api;
mod bookmark;
mod command;
mod environment;
mod interpreter;
mod os;
// mod scan;
mod storage;

fn main() {
  let environment = Environment::new();
  let program = environment.api.start();

  let return_code = match program.command {
    Command::AddClipboard {} => command_add_clipboard(&environment),
    Command::AddInput { input } => command_add_input(&environment, input),
    Command::List {} => command_list(&environment),
    Command::Clear { yes } => command_clear(&environment, yes),
  };

  process::exit(return_code)

  // let mut bookmarks = environment
  //   .storage
  //   .get_bookmarks(&environment)
  //   .map(|b| (score("box", &b), b))
  //   .collect::<Vec<(f64, Bookmark)>>();

  // println!("{:#?}", bookmarks);

  // bookmarks.sort_by(|(score1, _), (score2, _)| score1.cmp(score2));

  // let bookmarks = &bookmarks
  //   .into_iter()
  //   .map(|(_, b)| b)
  //   .collect::<Vec<Bookmark>>();

  // print!("{:?}", environment.api.select_bookmark(&bookmarks));
}

fn command_add_clipboard(environment: &Environment) -> i32 {
  let clipboard = environment.operating_system.get_clipboard();

  clipboard
    .and_then(|clipboard| {
      match_interpreter(environment, &clipboard).map(|interpreter| (clipboard, interpreter))
    })
    .and_then(|(clipboard, interpreter)| {
      let bookmark = interpreter.interpet(&environment, &clipboard);

      environment.storage.store_bookmark(&environment, &bookmark);

      Some(0)
    })
    // FIXME: add error reporting
    .unwrap_or(1)
}

fn command_add_input(environment: &Environment, input: String) -> i32 {
  match_interpreter(environment, &input)
    .and_then(|interpreter| {
      let bookmark = interpreter.interpet(&environment, &input);

      environment.storage.store_bookmark(&environment, &bookmark);

      Some(0)
    })
    // FIXME: add error reporting
    .unwrap_or(1)
}

fn command_list(environment: &Environment) -> i32 {
  environment
    .api
    .list_bookmarks(environment.storage.get_bookmarks(environment));

  0
}

fn command_clear(environment: &Environment, mut yes: bool) -> i32 {
  if !yes {
    yes = environment
      .api
      .confirm("Are you sure you want to clear bookmarks? THIS IS IRREVERSABLE!");
  }

  if yes {
    environment.storage.clear(environment);

    0
  } else {
    1
  }
}
