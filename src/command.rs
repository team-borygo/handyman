pub struct Program {
  pub command: Command,
}

pub enum Command {
  List {},
  AddClipboard {},
  AddSelection {},
  AddInput { input: String },
  Clear { yes: bool },
  Select { id: u32 },
  SelectInteractive {},
}
