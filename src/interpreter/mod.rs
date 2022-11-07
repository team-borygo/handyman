use crate::bookmark::Bookmark;

use self::text::TextInterpreter;

mod fs;
mod text;
mod url;

pub trait Interpreter {
    fn check(&self, input: &str) -> bool;
    fn interpet(&self, input: &str) -> Bookmark;
    fn belongs(&self, interpreted_by: &str) -> bool;
}

pub fn get_interpreters() -> Vec<Box<dyn Interpreter>> {
    vec![Box::new(TextInterpreter {})]
}
