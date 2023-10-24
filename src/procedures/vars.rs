use crate::data::{Error, Val};

pub fn var(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => Ok(Val::var(args[0].clone())),
        _ => Err(Error::Arity("var")),
    }
}

pub fn deref(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Var(v) => Ok(v.borrow().clone()),
            _ => Err(Error::ArgType("deref", "var", args[0].clone())),
        },
        _ => Err(Error::Arity("var")),
    }
}

pub fn set(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match &args[0] {
            Val::Var(v) => {
                v.replace(args[1].clone());
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("set!", "var", args[0].clone())),
        },
        _ => Err(Error::Arity("var")),
    }
}
