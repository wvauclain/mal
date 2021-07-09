use std::collections::BTreeMap;
use std::iter::Peekable;

use super::{Form, ParseError, Token};

pub struct Reader<'a> {
    input: Peekable<Box<dyn Iterator<Item = char> + 'a>>,
}

pub fn read_str(input: &String) -> Result<Form, ParseError> {
    let mut reader = Reader::new(input.chars()).peekable();
    return read_form(&mut reader);
}

fn peek_token<'a>(
    reader: &'a mut Peekable<Reader>,
    none_error: ParseError,
) -> Result<&'a Token, ParseError> {
    Ok(reader.peek().as_ref().ok_or(none_error)?.as_ref()?)
}

pub fn read_form(reader: &mut Peekable<Reader>) -> Result<Form, ParseError> {
    let token = reader.peek().ok_or(ParseError::MissingForm)?.as_ref()?;

    match token {
        Token::SpecialCharacter('[') => {
            reader.next();
            Ok(Form::Vector(read_list(
                reader,
                ']',
                ParseError::UnbalancedSquareBrackets,
            )?))
        }
        Token::SpecialCharacter(']') => Err(ParseError::UnexpectedCloseSquareBracket),
        Token::SpecialCharacter('{') => {
            reader.next();
            Ok(Form::Map(read_map(reader)?))
        }
        Token::SpecialCharacter('}') => Err(ParseError::UnexpectedCloseCurlyBracket),
        Token::SpecialCharacter('(') => {
            reader.next();
            Ok(Form::List(read_list(
                reader,
                ')',
                ParseError::UnbalancedParens,
            )?))
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
            } else if s.chars().nth(0) == Some(':') {
                Ok(Form::Keyword(s.as_str()[1..].to_owned()))
            } else {
                // TODO: we need to do more sophisticated things here eventually
                Ok(Form::Symbol(s.clone()))
            }
        }
    }
}

fn read_list(
    reader: &mut Peekable<Reader>,
    end_char: char,
    unbalanced_error: ParseError,
) -> Result<Vec<Form>, ParseError> {
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

fn read_map(reader: &mut Peekable<Reader>) -> Result<BTreeMap<Form, Form>, ParseError> {
    let mut map = BTreeMap::new();

    loop {
        let key = match peek_token(reader, ParseError::UnbalancedCurlyBrackets)? {
            Token::SpecialCharacter('}') => break,
            _ => read_form(reader)?,
        };
        reader.next();

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
) -> Result<Form, ParseError> {
    let mut l = vec![Form::Symbol(fun.as_ref().to_string())];

    for _ in 0..num_args {
        reader.next().ok_or(ParseError::MissingForm)??;
        l.push(read_form(reader)?)
    }

    Ok(Form::List(l))
}

/// with-meta uses its two arguments in reverse order, so we need to do this specially
fn with_meta(reader: &mut Peekable<Reader>) -> Result<Form, ParseError> {
    let mut l = Vec::new();
    l.resize(3, Form::Nil);

    l[0] = Form::Symbol("with-meta".to_owned());

    reader.next().ok_or(ParseError::MissingForm)??;
    l[2] = read_form(reader)?;

    reader.next().ok_or(ParseError::MissingForm)??;
    l[1] = read_form(reader)?;

    Ok(Form::List(l))
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
