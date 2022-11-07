use std::process::Command;
use std::str;

use super::OperatingSystem;

pub struct Linux {}

impl OperatingSystem for Linux {
  fn get_clipboard(&self) -> Option<String> {
    let output = Command::new("xsel")
      .arg("-o")
      .arg("-b")
      .output()
      .expect("failed to execute /xsel/");

    if output.stdout.len() == 0 {
      None
    } else {
      Some(
        str::from_utf8(&output.stdout)
          .expect("failed to parse clipboard content into UTF-8")
          .to_string(),
      )
    }
  }
}
