use crate::bookmark::Bookmark;
use environment::Environment;
use scan::score;

mod api;
mod bookmark;
mod environment;
mod interpreter;
mod os;
mod scan;
mod storage;

fn main() {
  let environment = Environment::new();

  let mut bookmarks = environment
    .storage
    .get_bookmarks(&environment)
    .map(|b| (score("box", &b), b))
    .collect::<Vec<(f64, Bookmark)>>();

  println!("{:#?}", bookmarks);

  // bookmarks.sort_by(|(score1, _), (score2, _)| score1.cmp(score2));

  // let bookmarks = &bookmarks
  //   .into_iter()
  //   .map(|(_, b)| b)
  //   .collect::<Vec<Bookmark>>();

  // print!("{:?}", environment.api.select_bookmark(&bookmarks));

  // let clipboard = environment.operating_system.get_clipboard();

  // clipboard
  //   .and_then(|clipboard| {
  //     println!("Clipboard content: {:?}", clipboard);

  //     environment
  //       .interpreters
  //       .iter()
  //       .find(|i| i.check(&clipboard))
  //       .map(|interpreter| (clipboard, interpreter))
  //   })
  //   .and_then(|(clipboard, interpreter)| {
  //     let bookmark = interpreter.interpet(&environment, &clipboard);

  //     environment.storage.store_bookmark(&environment, &bookmark);

  //     println!(
  //       "{:?}",
  //       environment
  //         .storage
  //         .get_bookmarks(&environment)
  //         .collect::<Vec<Bookmark>>()
  //     );

  //     Some(())
  //   });
}
