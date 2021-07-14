use std::error::Error;

use libmal::{printer, Atom, Compound, Form, Object};

use super::Environment;

pub fn eval(ast: Object, environment: &mut Environment) -> Result<Object, Box<dyn Error>> {
    println!("AST: {}", printer::pr_str(&*ast.borrow(), true));
    Ok(match &*ast.borrow() {
        Form::Compound(Compound::List(l)) if l.len() == 0 => ast.clone(),
        Form::Compound(Compound::List(_)) => {
            if let Form::Compound(Compound::List(l)) =
                &*eval_ast(ast.clone(), environment)?.borrow()
            {
                let result = l[0].call(&l[1..])?;
                println!("Result: {}", printer::pr_str(&*result.borrow(), true));
                result
            } else {
                panic!()
            }
        }
        _ => eval_ast(ast.clone(), environment)?,
    })
}

fn eval_ast(ast: Object, environment: &mut Environment) -> Result<Object, Box<dyn Error>> {
    Ok(match &*ast.borrow() {
        Form::Atom(Atom::Symbol(s)) => environment
            .get(s)
            .ok_or(format!("Symbol '{}' not found in the environment", s))?
            .clone(),
        Form::Compound(Compound::List(l)) => Form::list(
            l.into_iter()
                .map(|f| eval(f.clone(), environment))
                .collect::<Result<_, _>>()?,
        )
        .into(),
        Form::Compound(Compound::Vector(v)) => Form::vector(
            v.into_iter()
                .map(|f| eval(f.clone(), environment))
                .collect::<Result<_, _>>()?,
        )
        .into(),
        Form::Compound(Compound::Map(m)) => Form::map(
            // XXX: this is really inefficient; I might want to look into
            // immutable container libraries
            m.into_iter()
                .map(|(k, v)| Ok((k.clone(), eval(v.clone(), environment)?)))
                .collect::<Result<_, Box<dyn Error>>>()?,
        )
        .into(),
        _ => ast.clone(),
    })
}
