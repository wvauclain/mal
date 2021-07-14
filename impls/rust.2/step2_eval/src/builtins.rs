use libmal::{printer, Atom, Form, Object};

pub fn binary_operation(op: impl Fn(i64, i64) -> i64 + 'static) -> Object {
    Form::builtin(move |args| {
        if args.len() != 2 {
            panic!("incorrect number of arguments")
        }

        let (x, y) = (&*args[0].borrow(), &*args[1].borrow());

        match (x, y) {
            (Form::Atom(Atom::Number(x)), Form::Atom(Atom::Number(y))) => {
                Form::number(op(*x, *y)).into()
            }
            _ => panic!(
                "arguments have incorrect type: {} and {}",
                printer::pr_str(x, true),
                printer::pr_str(y, true)
            ),
        }
    })
    .into()
}
