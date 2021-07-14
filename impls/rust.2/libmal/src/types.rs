use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::errors::RuntimeError;

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

pub struct MalFn {
    // TODO: these functions need to handle failure, they should probably return RuntimeError
    pub exec: Box<dyn Fn(&[Object]) -> Object>,
    /// If a Mal function is defined inside Mal, this will hold the contents of
    /// the `defun` to be displayed upon request
    pub code: Option<Object>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum Atom {
    Number(i64),
    Symbol(String),
    String(String),
    Keyword(String),
    Nil,
    True,
    False,
}

pub enum Compound {
    List(Vec<Object>),
    Vector(Vec<Object>),
    Map(HashMap<Atom, Object>),
    Fn(MalFn),
}

pub enum Form {
    Atom(Atom),
    Compound(Compound),
}

#[derive(Clone)]
pub struct Object(Rc<RefCell<Form>>);

impl Form {
    pub const NIL: Form = Form::Atom(Atom::Nil);
    pub const TRUE: Form = Form::Atom(Atom::True);
    pub const FALSE: Form = Form::Atom(Atom::False);

    pub fn number(n: i64) -> Form {
        Form::Atom(Atom::Number(n))
    }

    pub fn symbol<S: AsRef<str>>(s: S) -> Form {
        Form::Atom(Atom::Symbol(s.as_ref().to_string()))
    }

    pub fn string<S: AsRef<str>>(s: S) -> Form {
        Form::Atom(Atom::String(s.as_ref().to_string()))
    }

    pub fn keyword<S: AsRef<str>>(s: S) -> Form {
        Form::Atom(Atom::Keyword(s.as_ref().to_string()))
    }

    pub fn list(l: Vec<Object>) -> Form {
        Form::Compound(Compound::List(l))
    }

    pub fn vector(v: Vec<Object>) -> Form {
        Form::Compound(Compound::Vector(v))
    }

    pub fn map(m: HashMap<Atom, Object>) -> Form {
        Form::Compound(Compound::Map(m))
    }

    pub fn builtin(f: impl Fn(&[Object]) -> Object + 'static) -> Form {
        Form::Compound(Compound::Fn(MalFn {
            exec: Box::new(f),
            code: None,
        }))
    }
}

impl Object {
    pub fn borrow(&self) -> std::cell::Ref<'_, Form> {
        self.0.borrow()
    }

    pub fn call(&self, args: &[Object]) -> Result<Object, RuntimeError> {
        if let Form::Compound(Compound::Fn(f)) = &*self.borrow() {
            Ok((f.exec)(args))
        } else {
            Err(RuntimeError::NotCallable)
        }
    }
}

impl From<Form> for Object {
    fn from(f: Form) -> Self {
        Object(Rc::new(RefCell::new(f)))
    }
}
