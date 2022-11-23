use crate::{
  bookmark::Bookmark,
  command::{Command, Program},
};
use clap::{ArgGroup, Parser, Subcommand};
use colored::*;

use super::Api;

pub struct Cli;
use dialoguer::{console::Term, Confirm, Input, Select};

impl Api for Cli {
  fn start(&self) -> Program {
    let cli = CliProgram::parse();

    Program {
      command: match cli.command {
        CliCommand::Add {
          clipboard,
          input,
          selection,
        } => {
          if (clipboard) {
            Command::AddClipboard {}
          } else if (selection) {
            Command::AddSelection {}
          } else {
            Command::AddInput {
              input: input.join(" "),
            }
          }
        }
        CliCommand::List {} => Command::List {},
        CliCommand::Clear { yes } => Command::Clear { yes },
        CliCommand::Select { id } => Command::Select { id },
      },
    }
  }

  fn prompt_text(&self, title: &str) -> String {
    Input::new().with_prompt(title).interact_text().unwrap()
  }

  fn select_bookmark<'a>(&self, bookmarks: &'a Vec<Bookmark>) -> &'a Bookmark {
    let selection = Select::new()
      .items(&bookmarks)
      .default(0)
      .interact_on_opt(&Term::stderr())
      .unwrap()
      .unwrap();

    &bookmarks[selection]
  }

  fn list_bookmarks(&self, iter: Box<dyn Iterator<Item = Bookmark>>) -> () {
    let mut i = 0;
    for bookmark in iter {
      println!(
        "{}. {} ({}...)",
        i,
        bookmark.title.green(),
        bookmark.content.short().dimmed()
      );

      i += 1;
    }
  }

  fn confirm(&self, title: &str) -> bool {
    Confirm::new().with_prompt(title).interact().unwrap()
  }

  fn print_error(&self, error: &str) -> () {
    eprintln!("{}", error.red());
  }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliProgram {
  #[command(subcommand)]
  command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
  #[command(group(
    ArgGroup::new("target")
      .required(true)
      .args(["clipboard", "selection", "input"])
  ))]
  Add {
    #[arg(short, long)]
    clipboard: bool,

    #[arg(short, long)]
    selection: bool,

    input: Vec<String>,
  },
  List {},
  Clear {
    #[arg(short)]
    yes: bool,
  },
  Select {
    id: u32,
  },
}
