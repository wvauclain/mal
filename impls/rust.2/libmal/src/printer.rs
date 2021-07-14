use super::{Atom, Compound, Form};

fn pr_compound(compound: &Compound, print_readably: bool) -> String {
    match compound {
        Compound::List(l) => {
            format!(
                "({})",
                l.into_iter()
                    .map(|f| pr_str(&*f.borrow(), print_readably))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        Compound::Vector(v) => {
            format!(
                "[{}]",
                v.into_iter()
                    .map(|f| pr_str(&*f.borrow(), print_readably))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        Compound::Map(m) => {
            format!(
                "{{{}}}",
                m.into_iter()
                    .map(|(k, v)| format!(
                        "{} {}",
                        pr_atom(k, print_readably),
                        pr_str(&*v.borrow(), print_readably)
                    ))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        Compound::Fn(f) => match &f.code {
            Some(code) => pr_str(&code.borrow(), print_readably),
            None => "[built-in function]".to_owned(),
        },
    }
}

pub fn pr_atom(atom: &Atom, print_readably: bool) -> String {
    match atom {
        Atom::Symbol(s) => s.clone(),
        Atom::String(s) => format!(
            r#""{}""#,
            if print_readably {
                s.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
            } else {
                s.clone()
            }
        ),
        Atom::Keyword(s) => format!(":{}", s),
        Atom::Number(n) => n.to_string(),
        Atom::Nil => "nil".to_owned(),
        Atom::True => "true".to_owned(),
        Atom::False => "false".to_owned(),
    }
}

pub fn pr_str(form: &Form, print_readably: bool) -> String {
    match form {
        Form::Atom(a) => pr_atom(a, print_readably),
        Form::Compound(c) => pr_compound(c, print_readably),
    }
}
