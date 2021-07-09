use std::collections::LinkedList;

mod errors;
pub use errors::*;

pub mod printer;
pub mod reader;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    /// The special two characters `~@`
    SpecialTwoCharacter,
    /// One of ``[]{}()'`~^@``
    SpecialCharacter(char),
    /// A double-quoted string
    String(String),
    /// A sequence of characters starting with a ;
    Comment(String),
    /// A sequence of one or more non-special characters
    CharacterSequence(String),
}

#[derive(Debug)]
pub enum Form {
    List(LinkedList<Form>),
    Number(i64),
    Symbol(String),
    String(String),
    Nil,
    True,
    False,
    // TODO: add more data types
}
