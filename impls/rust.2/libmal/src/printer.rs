use super::Form;

pub fn pr_str(form: &Form, print_readably: bool) -> String {
    match form {
        Form::List(l) => {
            format!(
                "({})",
                l.into_iter()
                    .map(|f| pr_str(f, print_readably))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        Form::Symbol(s) => s.clone(),
        Form::String(s) => format!(
            r#""{}""#,
            if print_readably {
                s.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
            } else {
                s.clone()
            }
        ),
        Form::Number(n) => n.to_string(),
        Form::Nil => "nil".to_owned(),
        Form::True => "true".to_owned(),
        Form::False => "false".to_owned(),
    }
}
