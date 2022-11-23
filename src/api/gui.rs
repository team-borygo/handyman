use crate::os::OperatingSystem;

use super::{cli::Cli, Api};

pub struct Gui {
  // FIXME: This should use the same OS as Environment
  // but circular reference is problematic
  operating_system: Box<dyn OperatingSystem>,
}

impl Gui {
  pub fn new(operating_system: Box<dyn OperatingSystem>) -> Gui {
    Gui { operating_system }
  }
}

impl Api for Gui {
  fn start(&self) -> crate::command::Program {
    // GUI has the same implementation for command-line as CLI
    Cli {}.start()
  }

  fn prompt_text(&self, title: &str) -> String {
    self.operating_system.prompt_text(title)
  }

  // FIXME: use some bookmark id for searching through list
  // This should be a better solution
  // Probably the implementation may be moved to Operating System
  fn select_bookmark<'a>(
    &self,
    bookmarks: &'a Vec<crate::bookmark::Bookmark>,
  ) -> &'a crate::bookmark::Bookmark {
    let bookmark_entries = bookmarks.iter().map(|b| b.to_string()).collect();
    let selected_bookmark = self
      .operating_system
      .interactive_select("Select bookmark", &bookmark_entries);

    bookmarks
      .iter()
      .find(|b| b.to_string() == selected_bookmark)
      // FIXME: what if there is no found bookmark
      .unwrap()
  }

  fn list_bookmarks(&self, _iter: Box<dyn Iterator<Item = crate::bookmark::Bookmark>>) -> () {
    eprintln!("List bookmarks is not implemeted for GUI mode")
  }

  fn confirm(&self, title: &str) -> bool {
    let entries = vec!["yes".to_string(), "no".to_string()];
    let selected_entry = self.operating_system.interactive_select(title, &entries);

    if selected_entry == "yes" {
      true
    } else {
      false
    }
  }

  fn print_error(&self, error: &str) -> () {
    eprintln!("{}", error)
  }
}
