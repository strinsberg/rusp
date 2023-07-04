use crate::data::*;
use std::rc::Rc;

// Procedure ///////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Procedure {
    pub name: Str,
    pub func: fn(&[Val]) -> Result<Val, Error>,
}

impl Procedure {
    pub fn new(name: &str, func: fn(&[Val]) -> Result<Val, Error>) -> Procedure {
        Procedure {
            name: Str::from(name),
            func: func,
        }
    }
}

// Representation //

impl DisplayRep for Procedure {
    fn to_display(&self) -> String {
        format!("#<procedure {}>", self.name.to_string())
    }
}

impl ExternalRep for Procedure {
    fn to_external(&self) -> String {
        format!("#<procedure {}>", self.name.to_string())
    }
}

impl std::fmt::Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Procedure{{ {} }}", self.to_external())
    }
}

// Equality //

impl PartialEq for Procedure {
    fn eq(&self, _other: &Procedure) -> bool {
        false
    }

    fn ne(&self, _other: &Procedure) -> bool {
        false
    }
}

impl Eq for Procedure {}

// Lambda /////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Lambda {
    pub name: Option<Str>,
    //pub params: Vec<&'static str>,
    func: Rc<dyn Fn(&[Val]) -> Result<Val, Error>>,
}

impl Lambda {
    pub fn new(
        name: &str,
        //params: Vec<&'static str>,
        func: Rc<dyn Fn(&[Val]) -> Result<Val, Error>>,
    ) -> Lambda {
        Lambda {
            name: Some(Str::from(name)),
            //params: params,
            func: func,
        }
    }

    pub fn call(&self, args: &[Val]) -> Result<Val, Error> {
        let f = &self.func;
        f(args)
    }
}

// Representation //

impl DisplayRep for Lambda {
    fn to_display(&self) -> String {
        match self.name.clone() {
            Some(name) => format!("#<closure {}>", name),
            None => format!("#<closure>",),
        }
    }
}

impl ExternalRep for Lambda {
    fn to_external(&self) -> String {
        match self.name.clone() {
            Some(name) => format!("#<closure {}>", name),
            None => format!("#<closure>",),
        }
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lambda{{ {} }}", self.to_external())
    }
}

impl PartialEq for Lambda {
    fn eq(&self, _other: &Lambda) -> bool {
        false
    }

    fn ne(&self, _other: &Lambda) -> bool {
        false
    }
}

impl Eq for Lambda {}

// Closure ////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct Closure {
    pub name: Option<Str>,
    pub env: Environ,
    pub formals: Formals,
    pub body: Option<Rc<List>>,
}

impl Closure {
    pub fn new(
        name: Option<Str>,
        env: Environ,
        formals: Formals,
        body: Option<Rc<List>>,
    ) -> Closure {
        Closure {
            name: name,
            env: env,
            formals: formals,
            body: body,
        }
    }

    pub fn tail_call(env: Environ, body: Val) -> Closure {
        Closure {
            name: None,
            env: env,
            formals: Formals::Fixed(Vec::new()),
            body: Some(Rc::new(List::new(body, None))),
        }
    }

    pub fn arity(&self) -> usize {
        match &self.formals {
            Formals::Collect(_) => 0,
            Formals::Fixed(vec) | Formals::Rest(vec, _) => vec.len(),
        }
    }
}

// Closure Traits /////////////////////////////////////////////////////////////

impl DisplayRep for Closure {
    fn to_display(&self) -> String {
        match self.name.clone() {
            Some(name) => format!("#<procedure {}>", name),
            None => format!("#<closure>",),
        }
    }
}

impl ExternalRep for Closure {
    fn to_external(&self) -> String {
        match self.name.clone() {
            Some(name) => format!("#<procedure {}>", name),
            None => format!("#<closure>",),
        }
    }
}

impl std::fmt::Display for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Closure{{ {} }}", self.to_external())
    }
}

impl Eq for Closure {}

// Formals ////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum Formals {
    Collect(Rc<Str>),
    Fixed(Vec<Rc<Str>>),
    Rest(Vec<Rc<Str>>, Rc<Str>),
}

// Tail Call //////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct TailCall {
    pub env: Environ,
    pub expr: Val,
}

impl TailCall {
    pub fn new(env: Environ, expr: Val) -> TailCall {
        TailCall { env, expr }
    }
}

// TailCall Traits /////////////////////////////////////////////////////////////

impl DisplayRep for TailCall {
    fn to_display(&self) -> String {
        format!("#<tail-call {}>", self.expr)
    }
}

impl ExternalRep for TailCall {
    fn to_external(&self) -> String {
        self.to_display()
    }
}

impl std::fmt::Display for TailCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for TailCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TailCall{{ {} }}", self.to_external())
    }
}

impl Eq for TailCall {}
