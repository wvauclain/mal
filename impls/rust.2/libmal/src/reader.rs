use std::collections::HashMap;
use std::iter::Peekable;

use super::{Atom, Form, Object, ParseError, Token};

pub struct Reader<'a> {
    input: Peekable<Box<dyn Iterator<Item = char> + 'a>>,
}

pub fn read_str(input: &String) -> Result<Object, ParseError> {
    let mut reader = Reader::new(input.chars()).peekable();
    return read_form(&mut reader);
}

fn peek_token<'a>(
    reader: &'a mut Peekable<Reader>,
    none_error: ParseError,
) -> Result<&'a Token, ParseError> {
    Ok(reader.peek().as_ref().ok_or(none_error)?.as_ref()?)
}

pub fn read_form(reader: &mut Peekable<Reader>) -> Result<Object, ParseError> {
    let token = reader.peek().ok_or(ParseError::MissingForm)?.as_ref()?;

    match token {
        Token::SpecialCharacter('[') => {
            reader.next();
            Ok(Form::vector(read_list(
                reader,
                ']',
                ParseError::UnbalancedSquareBrackets,
            )?)
            .into())
        }
        Token::SpecialCharacter(']') => Err(ParseError::UnexpectedCloseSquareBracket),
        Token::SpecialCharacter('{') => {
            reader.next();
            Ok(Form::map(read_map(reader)?).into())
        }
        Token::SpecialCharacter('}') => Err(ParseError::UnexpectedCloseCurlyBracket),
        Token::SpecialCharacter('(') => {
            reader.next();
            Ok(Form::list(read_list(reader, ')', ParseError::UnbalancedParens)?).into())
        }
        Token::SpecialCharacter(')') => Err(ParseError::UnexpectedCloseParen),
        Token::SpecialCharacter('\'') => Ok(call("quote", reader, 1)?),
        Token::SpecialCharacter('`') => Ok(call("quasiquote", reader, 1)?),
        Token::SpecialCharacter('~') => Ok(call("unquote", reader, 1)?),
        Token::SpecialCharacter('^') => Ok(with_meta(reader)?),
        Token::SpecialCharacter('@') => Ok(call("deref", reader, 1)?),
        Token::SpecialCharacter(c) => {
            panic!("somehow parsed invalid special character {}", c)
        }
        Token::SpecialTwoCharacter => Ok(call("splice-unquote", reader, 1)?),
        Token::Comment(_) => {
            reader.next();
            read_form(reader)
        }
        Token::String(s) => Ok(Form::string(s.clone()).into()),
        Token::CharacterSequence(s) => {
            if let Ok(num) = i64::from_str_radix(s.as_ref(), 10) {
                Ok(Form::number(num).into())
            } else if s == "nil" {
                Ok(Form::NIL.into())
            } else if s == "true" {
                Ok(Form::TRUE.into())
            } else if s == "false" {
                Ok(Form::FALSE.into())
            } else if s.chars().nth(0) == Some(':') {
                Ok(Form::keyword(&s.as_str()[1..]).into())
            } else {
                Ok(Form::symbol(s).into())
            }
        }
    }
}

fn read_list(
    reader: &mut Peekable<Reader>,
    end_char: char,
    unbalanced_error: ParseError,
) -> Result<Vec<Object>, ParseError> {
    let mut out = Vec::new();

    loop {
        let token = peek_token(reader, unbalanced_error)?;

        match token {
            Token::SpecialCharacter(c) if *c == end_char => break,
            _ => out.push(read_form(reader)?),
        }
        reader.next();
    }

    Ok(out)
}

fn read_map(reader: &mut Peekable<Reader>) -> Result<HashMap<Atom, Object>, ParseError> {
    let mut map = HashMap::new();

    loop {
        let key = match peek_token(reader, ParseError::UnbalancedCurlyBrackets)? {
            Token::SpecialCharacter('}') => break,
            _ => read_form(reader)?,
        };
        reader.next();

        let key = match &*key.borrow() {
            Form::Atom(a) => a.clone(),
            Form::Compound(_) => return Err(ParseError::UnexpectedCompound),
        };

        let value = match peek_token(reader, ParseError::UnbalancedCurlyBrackets)? {
            Token::SpecialCharacter('}') => return Err(ParseError::KeyWithoutValue),
            _ => read_form(reader)?,
        };
        reader.next();

        map.insert(key, value);
    }

    Ok(map)
}

fn call<S: AsRef<str>>(
    fun: S,
    reader: &mut Peekable<Reader>,
    num_args: u32,
) -> Result<Object, ParseError> {
    let mut l = vec![Form::symbol(fun.as_ref().to_string()).into()];

    for _ in 0..num_args {
        reader.next().ok_or(ParseError::MissingForm)??;
        l.push(read_form(reader)?)
    }

    Ok(Form::list(l).into())
}

/// with-meta uses its two arguments in reverse order, so we need to do this specially
fn with_meta(reader: &mut Peekable<Reader>) -> Result<Object, ParseError> {
    let mut l = Vec::new();

    l.push(Form::symbol("with-meta").into());

    reader.next().ok_or(ParseError::MissingForm)??;
    let arg2 = read_form(reader)?;

    reader.next().ok_or(ParseError::MissingForm)??;
    let arg1 = read_form(reader)?;

    l.push(arg1);
    l.push(arg2);

    Ok(Form::list(l).into())
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
                Some(c) if c == '\\' && self.input.peek() == Some(&'n') => {
                    out.push('\n');
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
                // Eat the '@'
                self.input.next();

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
