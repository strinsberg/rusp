use crate::data::{Error, Val};

pub fn throw(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2.. => {
            match args[0].clone() {
                Val::Keyword(_) => (),
                val => return Err(Error::ArgType("throw", "keyword", val)),
            };
            match args[1].clone() {
                Val::String(_) => (),
                val => return Err(Error::ArgType("throw", "string", val)),
            };
            Err(Error::Throw(
                args[0].clone(),
                args[1].clone(),
                args[2..].into(),
            ))
        }
        _ => Err(Error::Arity("throw")),
    }
}
