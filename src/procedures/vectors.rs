use crate::data::{Error, Num, Val, Vector};

// Construction and Manipulation //

pub fn vector(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Vector::from(args.to_vec())))
}

pub fn tuple(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Vector::tuple(args.to_vec())))
}

pub fn make_vector(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match &args[0] {
            Val::Number(Num::Int(n)) if n >= &0 => {
                Ok(Val::from(vec![Val::None; n.clone() as usize]))
            }
            _ => Err(Error::ArgType(
                "make-vector",
                "non-negative integer",
                args[0].clone(),
            )),
        },
        2 => match &args[0] {
            Val::Number(Num::Int(n)) if n >= &0 => {
                Ok(Val::from(vec![args[1].clone(); *n as usize]))
            }
            _ => Err(Error::ArgType(
                "make-vector",
                "non-negative integer",
                args[0].clone(),
            )),
        },
        _ => Err(Error::Arity("push")),
    }
}

pub fn push(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match args[0].clone() {
            Val::Vector(vec) if !vec.borrow().is_tuple() => {
                vec.borrow_mut().push(args[1].clone())?;
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("push", "vector", args[0].clone())),
        },
        _ => Err(Error::Arity("push")),
    }
}

pub fn pop(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(vec) if !vec.borrow().is_tuple() => {
                vec.borrow_mut().pop()?;
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("pop", "vector", args[0].clone())),
        },
        _ => Err(Error::Arity("pop")),
    }
}

// TODO could make this generic so that it works on strings, if they are supposed
// to be mutable.
pub fn set_nth(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        3 => match (&args[0], &args[1]) {
            (Val::Vector(vec), Val::Number(Num::Int(n))) => {
                if vec.borrow().is_tuple() {
                    return Err(Error::ArgType("set-nth", "vector", args[0].clone()));
                }
                if n < &0 {
                    return Err(Error::ArgType(
                        "set-nth!",
                        "non-negative integer",
                        args[1].clone(),
                    ));
                }
                vec.borrow_mut().set(args[2].clone(), *n as usize)?;
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("set-nth!", "vector", args[0].clone())),
        },
        _ => Err(Error::Arity("set-nth!")),
    }
}

// TODO could make this collection generic so that it works on strings
// that is if strings are mutable.
pub fn fill(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match args[0].clone() {
            Val::Vector(vec) if !vec.borrow().is_tuple() => {
                let n = vec.borrow().len();
                for i in 0..n {
                    vec.borrow_mut().set(args[1].clone(), i as usize)?;
                }
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("vector-fill!", "vector", args[0].clone())),
        },
        _ => Err(Error::Arity("vector-fill!")),
    }
}

// Conversion //

pub fn vector_to_tuple(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if !v.borrow().is_tuple() => {
                Ok(Val::from(Vector::copy_to_tuple(&*v.borrow())))
            }
            Val::Vector(v) if v.borrow().is_tuple() => Ok(args[0].clone()),
            _ => Err(Error::ArgType(
                "vector->tuple",
                "vector/tuple",
                args[0].clone(),
            )),
        },
        _ => Err(Error::Arity("vector->tuple")),
    }
}

pub fn tuple_to_vector(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if v.borrow().is_tuple() => {
                Ok(Val::from(Vector::copy_to_vec(&*v.borrow())))
            }
            _ => Err(Error::ArgType("tuple->vector", "tuple", args[0].clone())),
        },
        _ => Err(Error::Arity("tuple->vector")),
    }
}

// TODO make this a collections generic method since it will work for both
// Vectors and Maps and if it is called on a list or tuple or dict it does
// not really make a difference.
pub fn freeze(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if !v.borrow().is_tuple() => {
                v.borrow_mut().freeze();
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("tuple->vector", "tuple", args[0].clone())),
        },
        _ => Err(Error::Arity("tuple->vector")),
    }
}

// Predicates //

pub fn is_vector(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(_) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("vector?")),
    }
}

pub fn is_tuple(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if v.borrow().is_tuple() => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("tuple?")),
    }
}
