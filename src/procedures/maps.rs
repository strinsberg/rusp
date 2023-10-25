use crate::data::{Error, Map, Val};

// Construction and Manipulation //

pub fn table(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Map::map_from_vec(args)?))
}

pub fn dict(args: &[Val]) -> Result<Val, Error> {
    Ok(Val::from(Map::dict_from_vec(args)?))
}

pub fn get(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        2 => match &args[0] {
            Val::Map(m) => match m.borrow().get(args[1].clone()) {
                Some(v) => Ok(v),
                None => Ok(Val::None),
            },
            _ => Err(Error::ArgType("get", "table/dict", args[0].clone())),
        },
        _ => Err(Error::Arity("get")),
    }
}

pub fn assoc(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1.. => match &args[0] {
            Val::Map(m) if !m.borrow().is_dict() => {
                m.borrow_mut().add_pairs_from_vec(&args[1..])?;
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("assoc!", "table", args[0].clone())),
        },
        _ => Err(Error::Arity("assoc!")),
    }
}

pub fn dissoc(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1.. => match &args[0] {
            Val::Map(m) if !m.borrow().is_dict() => {
                for val in args[1..].iter() {
                    m.borrow_mut().dissoc(val.clone())?
                }
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("dissoc!", "table", args[0].clone())),
        },
        _ => Err(Error::Arity("dissoc!")),
    }
}

pub fn clear(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match &args[0] {
            Val::Map(m) => {
                m.borrow_mut().clear()?;
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("clear!", "table", args[0].clone())),
        },
        _ => Err(Error::Arity("clear!")),
    }
}

pub fn merge(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1.. => match &args[0] {
            Val::Map(m) if !m.borrow().is_dict() => {
                for val in args[1..].iter() {
                    match val {
                        Val::Map(other) => {
                            let mut map = m.borrow_mut();
                            for (k, v) in other.borrow().entries() {
                                map.assoc(k.clone(), v.clone())?
                            }
                        }
                        _ => return Err(Error::ArgType("merge!", "table", val.clone())),
                    }
                }
                Ok(args[0].clone())
            }
            _ => Err(Error::ArgType("merge!", "table", args[0].clone())),
        },
        _ => Err(Error::Arity("merge!")),
    }
}

// Predicates //

pub fn is_table(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match &args[0] {
            Val::Map(_) => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("table?")),
    }
}

pub fn is_dict(args: &[Val]) -> Result<Val, Error> {
    match args.len() {
        1 => match &args[0] {
            Val::Map(v) if v.borrow().is_dict() => Ok(Val::Bool(true)),
            _ => Ok(Val::Bool(false)),
        },
        _ => Err(Error::Arity("dict?")),
    }
}
