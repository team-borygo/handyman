pub struct Program {
  pub command: Command,
}

pub enum Command {
  List {},
  AddClipboard {},
  AddInput { input: String },
  Clear { yes: bool },
}
