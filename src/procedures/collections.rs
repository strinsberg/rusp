use crate::data::{Error, Num, Val};

pub fn nth(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => {
            let idx = match args[0].clone() {
                Val::Number(Num::Int(n)) if n >= 0 => n as usize,
                _ => {
                    return Err(Error::ArgType(
                        "nth",
                        "non-negative integer",
                        args[0].clone(),
                    ))
                }
            };
            match args[1].clone() {
                Val::List(ls) => match ls.get(idx) {
                    Some(val) => Ok(val),
                    None => Ok(Val::None),
                },
                Val::Vector(vec) => match vec.borrow().get(idx) {
                    Some(val) => Ok(val),
                    None => Ok(Val::None),
                },
                Val::Empty => Ok(Val::None),
                _ => return Err(Error::ArgType("nth", "list/vector/tuple", args[0].clone())),
            }
        }
        _ => Err(Error::Arity("nth")),
    }
}

pub fn length(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match args[0].clone() {
            Val::List(list) => Ok(Val::from(list.len() as i64)),
            Val::Vector(vec) => Ok(Val::from(vec.borrow().len() as i64)),
            Val::Map(map) => Ok(Val::from(map.borrow().len() as i64)),
            Val::Empty => Ok(Val::from(0)),
            _ => {
                return Err(Error::ArgType(
                    "length",
                    "list/vector/tuple/map/dict",
                    args[0].clone(),
                ))
            }
        },
        _ => Err(Error::Arity("length")),
    }
}

// Many of these could be easily made if letrec and do/loop were finished
// which need macros working better unless I want to do them in rust.
// Put freeze and fill
// make sure to include strings where applicable
// concat, as a non-destructive merge for map. though maybe we need both types
// for vector anyway.
// map, filter, reduce, for-each
