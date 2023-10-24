use crate::data::{Error, Vector, Val};

// Construction and Manipulation //

pub fn vector(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Vector::from(args.to_vec())))
}

pub fn tuple(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Vector::tuple(args.to_vec())))
}

pub fn push(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match args[0].clone() {
            Val::Vector(vec) if !vec.borrow().is_tuple() => {
                vec.borrow_mut().push(args[1].clone())?;
                Ok(args[0].clone())
            },
            _ => Err(Error::ArgType("push", "vector", args[0].clone())),
        }
        _ => Err(Error::Arity("push")),
    }
}

pub fn pop(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(vec) if !vec.borrow().is_tuple() => {
                vec.borrow_mut().pop()?;
                Ok(args[0].clone())
            },
            _ => Err(Error::ArgType("pop", "vector", args[0].clone())),
        }
        _ => Err(Error::Arity("pop")),
    }
}

// Conversion //

pub fn vector_to_tuple(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if !v.borrow().is_tuple() => {
                Ok(Val::from(Vector::copy_to_tuple(&*v.borrow())))
            },
            Val::Vector(v) if v.borrow().is_tuple() => Ok(args[0].clone()),
            _ => Err(Error::ArgType("vector->tuple", "vector/tuple", args[0].clone())),
        },
        _ => Err(Error::Arity("vector->tuple")),
    }
}

pub fn tuple_to_vector(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if v.borrow().is_tuple() => {
                Ok(Val::from(Vector::copy_to_vec(&*v.borrow())))
            },
            _ => Err(Error::ArgType("tuple->vector", "tuple", args[0].clone())),
        },
        _ => Err(Error::Arity("tuple->vector")),
    }
}

// TODO make this a collections generic method since it will work for both
// Vectors and Maps and if it is called on a list or tuple or dict it does
// not really make a difference.
pub fn vector_freeze(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::Vector(v) if !v.borrow().is_tuple() => {
                v.borrow_mut().freeze();
                Ok(args[0].clone())
            },
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
