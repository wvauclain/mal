use std::collections::BTreeMap;

mod errors;
pub use errors::*;

pub mod printer;
pub mod reader;

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Form {
    List(Vec<Form>),
    Vector(Vec<Form>),
    Map(BTreeMap<Form, Form>),
    Number(i64),
    Symbol(String),
    String(String),
    Keyword(String),
    Nil,
    True,
    False,
}
