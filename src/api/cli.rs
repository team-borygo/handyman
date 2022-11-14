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
        CliCommand::Add { clipboard, input } => match clipboard {
          true => Command::AddClipboard {},
          false => Command::AddInput {
            input: input.join(" "),
          },
        },
        CliCommand::List {} => Command::List {},
        CliCommand::Clear { yes } => Command::Clear { yes },
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
    for bookmark in iter {
      println!(
        "{} ({}...)",
        bookmark.title.green(),
        bookmark.content.short().dimmed()
      );
    }
  }

  fn confirm(&self, title: &str) -> bool {
    Confirm::new().with_prompt(title).interact().unwrap()
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
      .args(["clipboard", "input"])
  ))]
  Add {
    #[arg(short, long)]
    clipboard: bool,

    input: Vec<String>,
  },
  List {},
  Clear {
    #[arg(short)]
    yes: bool,
  },
}
