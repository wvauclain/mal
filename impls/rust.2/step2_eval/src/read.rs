use libmal::reader::read_str;
use libmal::{Object, ParseError};

pub fn read(input: String) -> Result<Object, ParseError> {
    read_str(&input)
}
