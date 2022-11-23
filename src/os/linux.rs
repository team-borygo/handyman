use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

use super::OperatingSystem;

pub struct Linux {}

impl OperatingSystem for Linux {
  fn get_selection(&self) -> Option<String> {
    let output = Command::new("xsel")
      .arg("-o")
      .arg("-p")
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

  fn write_clipboard(&self, input: &str) -> () {
    let mut input_command = Command::new("xsel")
      .arg("-b")
      .stdin(Stdio::piped())
      .spawn()
      .expect("failed to execute /xsel/");

    let stdin = input_command.stdin.as_mut().unwrap();

    stdin
      .write_all(input.as_bytes())
      .expect("failed to write to /xsel/ stdin");
  }

  fn ensure_storage_path(&self) -> std::path::PathBuf {
    let home = env::var("HOME").expect("HOME env variable is not set");
    let default_xdg_data = PathBuf::from(home).join("/.local/share");
    let xdg_data = env::var("XDG_DATA_HOME")
      .and_then(|p| Ok(PathBuf::from(p)))
      .unwrap_or(default_xdg_data);

    let directory = xdg_data.join("handyman");

    create_dir_all(&directory).expect("Cannot create directory for file storage");

    let path = directory.join("storage");

    if !path.exists() {
      File::create(&path).expect("Cannot create file for file storage");
    }

    path
  }

  fn prompt_text(&self, title: &str) -> String {
    let mut command = Command::new("dmenu")
      .arg("-p")
      .arg(title)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to execute /dmenu/");

    let stdin = command.stdin.as_mut().unwrap();
    stdin
      .write_all(b"")
      .expect("failed to write to /dmenu/ stdin");
    drop(stdin);

    let output = command.wait_with_output().unwrap();

    str::from_utf8(&output.stdout)
      .expect("failed to parse input data into UTF-8")
      .trim()
      .to_string()
  }

  fn interactive_select(&self, title: &str, options: &Vec<String>) -> String {
    let mut command = Command::new("dmenu")
      .arg("-l")
      .arg("10")
      .arg("-p")
      .arg(title)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to execute /dmenu/");

    let stdin = command.stdin.as_mut().unwrap();
    stdin
      .write_all(options.join("\n").as_bytes())
      .expect("failed to write to /dmenu/ stdin");
    drop(stdin);

    let output = command.wait_with_output().unwrap();

    str::from_utf8(&output.stdout)
      .expect("failed to parse input data into UTF-8")
      .trim()
      .to_string()
  }
}
