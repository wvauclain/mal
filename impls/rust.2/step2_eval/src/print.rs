use libmal::printer::pr_str;
use libmal::Object;

pub fn print(input: Object) -> String {
    pr_str(&*input.borrow(), true)
}
