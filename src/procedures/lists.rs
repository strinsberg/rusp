use crate::data::{Error, List, Val};

// Construction //

pub fn cons(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => Ok(Val::from(List::new(args[0].clone(), None))),
        2 => Ok(Val::from(List::cons(args[0].clone(), args[1].clone()))),
        _ => Err(Error::Arity("cons")),
    }
}

pub fn list(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::list_from_vec(args))
}

// Simple Access //

pub fn rest(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::List(list) => match list.tail() {
                Some(ls) => Ok(Val::List(ls)),
                None => Ok(Val::Empty),
            },
            Val::Empty => Ok(Val::Empty),
            _ => return Err(Error::ArgType("rest", "list", args[0].clone())),
        },
        _ => Err(Error::Arity("rest")),
    }
}

// Information //

pub fn is_list(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::List(_) | Val::Empty => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("list?")),
    }
}

pub fn is_null(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Empty => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("list?")),
    }
}
