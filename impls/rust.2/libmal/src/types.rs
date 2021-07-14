use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub exec: Box<dyn Fn(Vec<Object>) -> Object>,
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

    pub fn mal_fn(f: MalFn) -> Form {
        Form::Compound(Compound::Fn(f))
    }
}

impl Object {
    pub fn borrow(&self) -> std::cell::Ref<'_, Form> {
        self.0.borrow()
    }

}

impl From<Form> for Object {
    fn from(f: Form) -> Self {
        Object(Rc::new(RefCell::new(f)))
    }
}
