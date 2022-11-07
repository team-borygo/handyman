use crate::bookmark::Bookmark;

use super::Interpreter;

pub struct TextInterpreter {}

impl TextInterpreter {
    fn get_name(&self) -> String {
        "text".to_string()
    }
}

impl Interpreter for TextInterpreter {
    fn check(&self, input: &str) -> bool {
        true
    }

    fn interpet(&self, input: &str) -> crate::bookmark::Bookmark {
        Bookmark::new(input.to_string(), input.to_string(), self.get_name())
    }

    fn belongs(&self, interpreted_by: &str) -> bool {
        interpreted_by == "text"
    }
}
