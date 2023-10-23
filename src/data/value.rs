use crate::data::*;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub enum Val {
    Symbol(Rc<Str>),
    Bool(bool),
    Number(Num),
    Keyword(Rc<Str>),
    Char(Char),
    String(Rc<RefCell<Str>>),
    List(Rc<List>),
    Vector(Rc<RefCell<Vector>>),
    Map(Rc<RefCell<Map>>),
    Procedure(Rc<Procedure>),
    Closure(Rc<Closure>),
    Lambda(Rc<Lambda>),
    Macro(Rc<Macro>),
    Empty,
    None,
    // not available to user
    TailCall(Rc<TailCall>),
    Undefined,
}

impl Val {
    // Constructors //
    pub fn list_from_vec(vec: &[Val]) -> Val {
        match List::from_vec(vec) {
            Some(ls) => Val::from(ls),
            None => Val::Empty,
        }
    }

    pub fn symbol(s: &str) -> Val {
        Val::Symbol(Rc::new(Str::from(s)))
    }

    pub fn keyword(s: &str) -> Val {
        if s.starts_with(':') {
            Val::Keyword(Rc::new(Str::from(s)))
        } else {
            Val::Keyword(Rc::new(Str::from(format!(":{s}").as_str())))
        }
    }

    pub fn tail_call(env: Environ, expr: Val) -> Val {
        Val::TailCall(Rc::new(TailCall::new(env, expr)))
    }

    // Predicates //

    pub fn is_true(&self) -> bool {
        match self {
            Val::Bool(false) | Val::None => false,
            _ => true,
        }
    }

    pub fn is_hashable(&self) -> bool {
        match self {
            Val::Bool(_)
            | Val::Keyword(_)
            | Val::Char(_)
            | Val::Number(_)
            | Val::Symbol(_)
            | Val::String(_) => true,
            _ => false,
        }
    }

    // Conversion //

    pub fn as_vec(&self) -> Vec<Val> {
        match self {
            Val::List(list) => list.values().map(|v| v.clone()).collect(),
            Val::Vector(vec) => vec.borrow().values().cloned().collect(),
            Val::Map(map) => map
                .borrow()
                .entries()
                .map(|(k, v)| Val::from(Vector::tuple(vec![k.clone(), v.clone()])))
                .collect(),
            val => vec![val.clone()],
        }
    }
}

// Val Traits /////////////////////////////////////////////////////////////

// From Traits/Constructors //

impl Default for Val {
    fn default() -> Val {
        Val::Empty
    }
}

impl From<i64> for Val {
    fn from(i: i64) -> Val {
        Val::Number(Num::Int(i))
    }
}

impl From<f64> for Val {
    fn from(f: f64) -> Val {
        Val::Number(Num::Flt(f))
    }
}

impl From<(i64, i64)> for Val {
    fn from(val: (i64, i64)) -> Val {
        Val::Number(Num::Rat(val.0, val.1))
    }
}

impl From<char> for Val {
    fn from(ch: char) -> Val {
        Val::Char(Char::from(ch))
    }
}

impl From<Str> for Val {
    fn from(s: Str) -> Val {
        Val::String(Rc::new(RefCell::new(s)))
    }
}

impl From<&str> for Val {
    fn from(s: &str) -> Val {
        Val::String(Rc::new(RefCell::new(Str::from(s))))
    }
}

impl From<List> for Val {
    fn from(list: List) -> Val {
        Val::List(Rc::new(list))
    }
}

impl From<Vec<Val>> for Val {
    fn from(vec: Vec<Val>) -> Val {
        Val::Vector(Rc::new(RefCell::new(Vector::from(vec))))
    }
}

impl From<Vector> for Val {
    fn from(vec: Vector) -> Val {
        Val::Vector(Rc::new(RefCell::new(vec)))
    }
}

impl From<Map> for Val {
    fn from(map: Map) -> Val {
        Val::Map(Rc::new(RefCell::new(map)))
    }
}

impl From<Procedure> for Val {
    fn from(p: Procedure) -> Val {
        Val::Procedure(Rc::new(p))
    }
}

impl From<Lambda> for Val {
    fn from(p: Lambda) -> Val {
        Val::Lambda(Rc::new(p))
    }
}

impl From<Closure> for Val {
    fn from(c: Closure) -> Val {
        Val::Closure(Rc::new(c))
    }
}

impl From<Macro> for Val {
    fn from(m: Macro) -> Val {
        Val::Macro(Rc::new(m))
    }
}

// Representations //

impl DisplayRep for Val {
    fn to_display(&self) -> String {
        match self {
            Val::Bool(val) => format!("#{}", if *val { "t" } else { "f" }),
            Val::Char(val) => val.to_display(),
            Val::Number(val) => val.to_display(),
            Val::Symbol(val) => val.to_string(),
            Val::Keyword(val) => val.to_string(),
            Val::String(val) => val.borrow().to_display(),
            Val::List(val) => val.to_display(),
            Val::Vector(val) => val.borrow().to_display(),
            Val::Map(val) => val.borrow().to_display(),
            Val::Procedure(p) => p.to_display(),
            Val::Lambda(c) => c.to_display(),
            Val::Closure(c) => c.to_display(),
            Val::Macro(m) => m.to_display(),
            Val::TailCall(t) => t.to_display(),
            Val::Empty => "#()".to_string(),
            Val::None => "#none".to_string(),
            Val::Undefined => "#<undefined>".to_string(),
        }
    }
}

impl ExternalRep for Val {
    fn to_external(&self) -> String {
        match self {
            Val::Bool(val) => format!("#{}", if *val { "t" } else { "f" }),
            Val::Char(val) => val.to_external(),
            Val::Number(val) => val.to_external(),
            Val::Symbol(val) => val.to_string(),
            Val::Keyword(val) => val.to_string(),
            Val::String(val) => val.borrow().to_external(),
            Val::List(val) => val.to_external(),
            Val::Vector(val) => val.borrow().to_external(),
            Val::Map(val) => val.borrow().to_external(),
            Val::Procedure(p) => p.to_external(),
            Val::Lambda(c) => c.to_display(),
            Val::Closure(c) => c.to_external(),
            Val::Macro(m) => m.to_external(),
            Val::TailCall(t) => t.to_external(),
            Val::Empty => "#()".to_string(),
            Val::None => "#none".to_string(),
            Val::Undefined => "#<undefined>".to_string(),
        }
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Val{{ {} }}", self.to_external())
    }
}

// Hashing //

impl Hash for Val {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Val::Bool(b) => b.hash(state),
            Val::Char(ch) => ch.hash(state),
            Val::Number(n) => n.hash(state),
            Val::Symbol(sym) => sym.hash(state),
            Val::Keyword(sym) => sym.hash(state),
            Val::String(s) => s.borrow().hash(state),
            _ => panic!("cannot hash this value type: {:?}", self),
        }
    }
}
