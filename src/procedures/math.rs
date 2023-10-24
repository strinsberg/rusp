use crate::data::{Error, Num, Val};

// Type Predicates //

pub fn is_number(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Number(_) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("=")),
    }
}

pub fn is_float(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Number(Num::Flt(_)) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("float?")),
    }
}

pub fn is_int(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Number(Num::Int(_)) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("integer?")),
    }
}

pub fn is_rational(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Number(Num::Rat(_, _)) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("rational?")),
    }
}

// Comparisson //

pub fn equals(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match (args[0].clone(), args[1].clone()) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Bool(a == b)),
            _ => Err(Error::ArgType("=", "numbers", Val::list_from_vec(args))),
        },
        _ => Err(Error::Arity("=")),
    }
}

pub fn less_than(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match (args[0].clone(), args[1].clone()) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Bool(a < b)),
            _ => Err(Error::ArgType("<", "numbers", Val::list_from_vec(args))),
        },
        _ => Err(Error::Arity("<")),
    }
}

pub fn greater_than(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match (args[0].clone(), args[1].clone()) {
            (Val::Number(a), Val::Number(b)) => Ok(Val::Bool(a > b)),
            _ => Err(Error::ArgType(">", "numbers", Val::list_from_vec(args))),
        },
        _ => Err(Error::Arity(">")),
    }
}

// Arithmetic //

pub fn add(left: &Val, right: &Val) -> Result<Val, Error> {
    match (left, right) {
        (Val::Number(l), Val::Number(r)) => Ok(Val::Number(l.add(r)?)),
        (Val::Number(_), v) => Err(Error::ArgType("add", "number", v.clone())),
        (v, Val::Number(_)) => Err(Error::ArgType("add", "number", v.clone())),
        _ => Err(Error::ArgType(
            "add",
            "number",
            Val::from(vec![left.clone(), right.clone()]),
        )),
    }
}

pub fn sum(args: &[Val]) -> Result<Val, Error> {
    args.iter().fold(Ok(Val::from(0)), |acc, x| add(&acc?, x))
}
