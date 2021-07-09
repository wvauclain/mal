use std::collections::LinkedList;
use std::iter::Peekable;

use super::{Form, ParseError, Token};

pub struct Reader<'a> {
    input: Peekable<Box<dyn Iterator<Item = char> + 'a>>,
}

pub fn read_str(input: &String) -> Result<Form, ParseError> {
    let mut reader = Reader::new(input.chars()).peekable();
    return read_form(&mut reader);
}

pub fn read_form(reader: &mut Peekable<Reader>) -> Result<Form, ParseError> {
    let token = reader.peek().ok_or(ParseError::MissingForm)?.as_ref()?;

    match token {
        Token::SpecialCharacter('(') => {
            reader.next();
            Ok(Form::List(read_list(reader)?))
        }
        Token::SpecialCharacter(c) => {
            panic!("Found unimplemented special character {}", c)
        }
        Token::SpecialTwoCharacter => {
            panic!("Unimplemented special two-character found")
        }
        Token::Comment(_) => {
            panic!("Can't handle comments yet")
        }
        Token::String(s) => Ok(Form::String(s.clone())),
        Token::CharacterSequence(s) => {
            if let Ok(num) = i64::from_str_radix(s.as_ref(), 10) {
                Ok(Form::Number(num))
            } else if s == "nil" {
                Ok(Form::Nil)
            } else if s == "true" {
                Ok(Form::True)
            } else if s == "false" {
                Ok(Form::False)
            } else {
                // TODO: we need to do more sophisticated things here eventually
                Ok(Form::Symbol(s.clone()))
            }
        }
    }
}

pub fn read_list(reader: &mut Peekable<Reader>) -> Result<LinkedList<Form>, ParseError> {
    let mut out = LinkedList::new();

    loop {
        // XXX: add proper error handling here
        let token = reader
            .peek()
            .as_ref()
            .ok_or(ParseError::UnbalancedParens)?
            .as_ref()?;

        match token {
            Token::SpecialCharacter(')') => break,
            _ => out.push_back(read_form(reader)?),
        }
        reader.next();
    }

    Ok(out)
}

impl<'a> Reader<'a> {
    pub fn new(input: impl Iterator<Item = char> + 'a) -> Self {
        Reader {
            input: std::iter::Iterator::peekable(Box::new(input)),
        }
    }

    fn consume_string(&mut self) -> Result<Token, ParseError> {
        let mut out = String::new();

        loop {
            match self.input.next() {
                None => {
                    return Err(ParseError::UnbalancedString);
                }
                Some(c) if c == '"' => return Ok(Token::String(out)),
                Some(c) if c == '\\' && self.input.peek() == Some(&'"') => {
                    out.push('"');
                    // Advance again to get past the escaped double-quote
                    self.input.next();
                }
                Some(c) if c == '\\' && self.input.peek() == Some(&'\\') => {
                    out.push('\\');
                    // Advance again to get past the escaped double-quote
                    self.input.next();
                }
                Some(c) => out.push(c),
            };
        }
    }

    fn consume_until(&mut self, first: char, pred: impl Fn(char) -> bool) -> String {
        let mut out = String::new();
        out.push(first);

        loop {
            match self.input.peek() {
                None => break,
                Some(c) if pred(*c) => break,
                Some(c) => {
                    out.push(*c);
                    self.input.next();
                }
            }
        }

        out
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.peek() {
                None => return None,
                Some(c) if !c.is_whitespace() && *c != ',' => break,
                Some(_) => self.input.next(),
            };
        }

        match self.input.next() {
            None => None,
            Some(c) if c == '~' && self.input.peek() == Some(&'@') => {
                Some(Ok(Token::SpecialTwoCharacter))
            }
            Some(c) if "[]{}()'`~^@".contains(c) => Some(Ok(Token::SpecialCharacter(c))),
            Some(c) if c == '"' => Some(self.consume_string()),
            Some(c) if c == ';' => Some(Ok(Token::Comment(self.consume_until(c, |c| c == '\n')))),
            Some(c) => {
                Some(Ok(Token::CharacterSequence(self.consume_until(c, |c| {
                    c.is_whitespace() || ",[]{}()'`~^@".contains(c)
                }))))
            }
        }
    }
}
