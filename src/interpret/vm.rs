use crate::data::*;
use std::rc::Rc;

// TODO implement proper defines for functions
// TODO make sure everything in the vm is tested
// TODO the reader needs to create the right collections given the new syntax
// TODO ensure that syntax expectations are appropriate, i.e. if we use vectors
// for lamnda and defn arguments or use vectors to wrap let bindings that can be
// done with lists. I think overall if I write most things in rusp with macros etc.
// it is easier to have forms keep the more verbose scheme like syntax for things
// like let. However, I like using vector literals with [] syntax in some places
// to help visually represent different structures.
// TODO test somewhere to ensure that we have the expected tail call semantics
// that prevent blowing up the stack when recursing. Some of this will require
// setting up macros the right way to use the parts of the language that are
// currently already passing back tail calls (lambda and if).
// TODO ensure that all structures are fully tested and working as expected with
// as many methods for easy procedure creation as possible.

pub struct Vm {
    env: Environ,
}

impl Vm {
    pub fn new(env: Environ) -> Vm {
        Vm { env: env }
    }

    // Evaluate forms /////////////////////////////////////////////////////////

    pub fn eval_forms(&mut self, forms: &[Val]) -> Result<Val, Error> {
        if forms.len() < 1 {
            return Ok(Val::None);
        }

        for f in forms[..forms.len() - 1].iter() {
            self.eval_top_level(f.clone())?;
        }
        self.eval_top_level(forms[forms.len() - 1].clone())
    }

    pub fn eval_top_level(&mut self, form: Val) -> Result<Val, Error> {
        //println!("{form}");
        match form {
            Val::List(ref ls) => match ls.head() {
                Val::Symbol(s) if s.is("def") => self.eval_define(ls.tail()),
                Val::Symbol(s) if s.is("macro-rules") => self.eval_macro_rules(ls.tail()),
                _ => self.eval(form, self.env.clone()),
            },
            _ => self.eval(form, self.env.clone()),
        }
    }

    fn eval(&self, form: Val, env: Environ) -> Result<Val, Error> {
        let mut expr = form.clone();
        let mut env = env.clone();

        loop {
            expr = match expr {
                Val::Symbol(s) => match env.lookup(&s) {
                    Some(v) => v,
                    None => return Err(Error::Undeclared(s.to_string())),
                },
                Val::List(ref ls) => match ls.head().clone() {
                    Val::Symbol(s) if self.is_special_form(s.clone()) => {
                        self.eval_special(s.to_string().as_str(), ls.tail(), env)?
                    }
                    Val::Symbol(_) | Val::Procedure(_) | Val::Closure(_) | Val::List(_) => {
                        self.eval_call(ls.clone(), env)?
                    }
                    Val::Macro(m) => m.expand(expr, env.clone())?,
                    head => return Err(Error::NotAProcedure(head)),
                },
                // TODO do we really want to eval a vector when we see it? or
                // do we want to eval vector literals when we see them only.
                // Is it even possible to eval it twice without being explicit
                // about it?
                // The reader transforms #(1 2 3) -> (list 1 2 3) which is why
                // those are evaluated properly. We could do that to ensure
                // vector literals are evalled properly when seen in code, but that
                // if encountered other ways are simply returned. I guess the
                // question is whether there is supposed to be a difference
                // between (vector ...) and [...]
                Val::Vector(v) => {
                        let evalled = v.borrow().values()
                        .map(|val| self.eval(val.clone(), env.clone()))
                        .collect::<Result<Vec<Val>, Error>>()?;
                    if v.borrow().is_tuple() {
                        Val::from(Vector::tuple(evalled))
                    } else {
                        Val::from(Vector::from(evalled))
                    }
                }
                // TODO add an eval for map literals
                Val::TailCall(_) | Val::Undefined => panic!("should not be evaluated: {expr}"),
                _ => expr,
            };

            // if it is a tail call resolve it, otherwise return the result
            match expr {
                Val::TailCall(tail) => {
                    env = tail.env.clone();
                    expr = tail.expr.clone();
                }
                _ => return Ok(expr),
            }
        }
    }

    // Eval Helpers ///////////////////////////////////////////////////////////

    fn eval_define(&mut self, list: Option<Rc<List>>) -> Result<Val, Error> {
        let ls = list.ok_or(Error::Arity("def"))?;
        let name = match ls.first().ok_or(Error::Arity("def"))? {
            Val::Symbol(s) => s,
            val => return Err(Error::ArgType("def", "symbol", val)),
        };
        let value = ls.second().ok_or(Error::Arity("def"))?;

        // TODO this ignores a defn or scheme like function define
        self.env.insert(name.clone(), Val::Undefined);
        self.env
            .insert(name.clone(), self.eval(value, self.env.clone())?);
        Ok(Val::None)
    }

    fn eval_macro_rules(&mut self, list: Option<Rc<List>>) -> Result<Val, Error> {
        // get the macro name and ensure it has arguments
        let ls = list.ok_or(Error::Arity("macro-rules"))?;
        let name = match ls.first().ok_or(Error::Arity("macro-rules"))? {
            Val::Symbol(s) => s,
            val => return Err(Error::ArgType("macro-rules", "symbol", val)),
        };

        // get the arg vector of symbol literals
        let args = ls.tail().ok_or(Error::Arity("macro-rules"))?;
        let syms = match args.first().ok_or(Error::Arity("macro-rules"))? {
            Val::Vector(v) => v
                .borrow()
                .values()
                .map(|val| match val {
                    Val::Symbol(s) => Ok(s.clone()),
                    _ => return Err(Error::ArgType("macro-rules", "symbol", val.clone())),
                })
                .collect::<Result<Vec<Rc<Str>>, Error>>()?,
            val => return Err(Error::ArgType("macro-rules", "symbol", val)),
        };

        // get list of rules and create the macro
        let rules = args.tail().ok_or(Error::Arity("macro-rules"))?;
        let mac = Val::from(Macro::new(name.clone(), &syms, rules)?);
        self.env.insert(name, mac);
        Ok(Val::None)
    }

    fn is_special_form(&self, s: Rc<Str>) -> bool {
        match s.to_string().as_str() {
            "if" | "lambda" => true,
            _ => false,
        }
    }

    fn eval_special(&self, name: &str, list: Option<Rc<List>>, env: Environ) -> Result<Val, Error> {
        match list {
            Some(ls) => match name {
                "if" => self.eval_if(ls, env),
                "lambda" => self.eval_lambda(ls, env),
                _ => panic!("not a special form: {name}"),
            },
            None => panic!("empty special form"),
        }
    }

    fn eval_if(&self, list: Rc<List>, env: Environ) -> Result<Val, Error> {
        let cond = list.first().ok_or(Error::Arity("if"))?;
        let true_branch = list.second().ok_or(Error::Arity("if"))?;
        let false_branch = list.third();

        if self.eval(cond, env.clone())?.is_true() {
            Ok(Val::tail_call(env, true_branch))
        } else {
            match false_branch {
                Some(expr) => Ok(Val::tail_call(env, expr)),
                None => Ok(Val::None),
            }
        }
    }

    fn eval_lambda(&self, list: Rc<List>, env: Environ) -> Result<Val, Error> {
        let formals = match list.head().clone() {
            Val::Symbol(s) => Formals::Collect(s),
            Val::Vector(vec) => self.formals_from_vector(vec.borrow().values())?,
            Val::Empty => Formals::Fixed(vec![]),
            _ => {
                return Err(Error::ArgType(
                    "lambda",
                    "symbol or vector of symbols",
                    list.head().clone(),
                ))
            }
        };
        Ok(Val::from(Closure::new(None, env, formals, list.tail())))
    }

    fn formals_from_vector(&self, args: std::slice::Iter<'_, Val>) -> Result<Formals, Error> {
        let mut vec = Vec::new();
        let mut rest = false;
        for val in args {
            match val {
                Val::Symbol(s) => {
                    if rest {
                        return Ok(Formals::Rest(vec, s.clone()));
                    } else if s.is(".") {
                        rest = true
                    } else {
                        vec.push(s.clone())
                    }
                }
                _ => return Err(Error::BadArg(1)),
            };
        }
        Ok(Formals::Fixed(vec))
    }

    fn eval_call(&self, list: Rc<List>, env: Environ) -> Result<Val, Error> {
        let first = match list.first() {
            Some(val) => self.eval(val, env.clone())?,
            None => panic!("function call should not be empty: {list}"),
        };

        match first {
            Val::Macro(m) => Ok(Val::tail_call(
                env.clone(),
                m.expand(Val::List(list), env.clone())?,
            )),
            _ => {
                let args = match list.tail() {
                    Some(ls) => ls
                        .values()
                        .map(|val| self.eval(val, env.clone()))
                        .collect::<Result<Vec<Val>, Error>>()?,
                    None => vec![],
                };
                match first {
                    Val::Procedure(p) => {
                        let f = p.func;
                        f(&args)
                    }
                    Val::Closure(c) => self.apply_closure(c.clone(), &args),
                    _ => Err(Error::NotAProcedure(first)),
                }
            }
        }
    }

    // Applications ///////////////////////////////////////////////////////////

    fn apply_closure(&self, closure: Rc<Closure>, args: &[Val]) -> Result<Val, Error> {
        // Bind the arguments to their parameters according to the formals list
        let bound_env = Env::add_scope(closure.env.clone());
        match closure.formals.clone() {
            Formals::Collect(symbol) => {
                bound_env.insert(symbol.clone(), Val::list_from_vec(args));
            }
            Formals::Fixed(params) => {
                if args.len() >= params.len() {
                    for (i, symbol) in params.iter().enumerate() {
                        bound_env.insert(symbol.clone(), args[i].clone());
                    }
                } else {
                    return Err(Error::Arity("closure"));
                }
            }
            Formals::Rest(params, symbol) => {
                let mut full_params = params.clone();
                full_params.push(symbol);

                let mut full_args: Vec<Val> = args[..params.len()].into();
                let rest = Val::list_from_vec(args[params.len()..].into());
                full_args.push(rest);

                for (i, symbol) in full_params.iter().enumerate() {
                    bound_env.insert(symbol.clone(), full_args[i].clone());
                }
            }
        };

        // Eval all body exressions but the last one
        match closure.body {
            Some(ref ls) => {
                for (i, val) in ls.values().enumerate() {
                    if i == ls.len() - 1 {
                        return Ok(Val::tail_call(bound_env, val));
                    } else {
                        self.eval(val, bound_env.clone())?;
                    }
                }
                unreachable!();
            }
            None => Ok(Val::None),
        }
    }
}

// Testing ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpret::null_env;

    #[test]
    fn test_self_evaluating_data() {
        let mut vm = Vm::new(null_env());
        assert_eq!(vm.eval_top_level(Val::Empty), Ok(Val::Empty));
        assert_eq!(vm.eval_top_level(Val::None), Ok(Val::None));
        assert_eq!(vm.eval_top_level(Val::Bool(true)), Ok(Val::Bool(true)));
        assert_eq!(vm.eval_top_level(Val::from(10)), Ok(Val::from(10)));
        assert_eq!(vm.eval_top_level(Val::from('h')), Ok(Val::from('h')));
        assert_eq!(
            vm.eval_top_level(Val::from("some string")),
            Ok(Val::from("some string"))
        );
        assert_eq!(
            vm.eval_top_level(Val::keyword(":hello")),
            Ok(Val::keyword(":hello"))
        );
        assert_eq!(
            vm.eval_top_level(Val::from(vec![Val::from(10)])),
            Ok(Val::from(vec![Val::from(10)]))
        );
        assert_eq!(
            vm.eval_top_level(Val::from(
                Map::map(&[(Val::from(10), Val::from(5))]).unwrap()
            )),
            Ok(Val::from(
                Map::map(&[(Val::from(10), Val::from(5))]).unwrap()
            ))
        );

        let proc = Val::from(Procedure::new("name", |_args| Ok(Val::None)));
        assert_eq!(vm.eval_top_level(proc.clone()), Ok(proc.clone()));

        let closure = Val::from(Closure::new(
            None,
            vm.env.clone(),
            Formals::Fixed(vec![]),
            Some(Rc::new(List::new(Val::None, None))),
        ));
        assert_eq!(vm.eval_top_level(closure.clone()), Ok(closure.clone()));
    }

    #[test]
    #[should_panic]
    fn test_evaluating_undefined() {
        let mut vm = Vm::new(null_env());
        vm.eval_top_level(Val::Undefined).err();
    }

    #[test]
    #[should_panic]
    fn test_evaluating_tail_call_directly() {
        let mut vm = Vm::new(null_env());
        vm.eval_top_level(Val::tail_call(vm.env.clone(), Val::from(10)))
            .err();
    }

    #[test]
    fn test_evaluating_define() {
        let mut vm = Vm::new(null_env());
        let expr = Val::list_from_vec(&vec![Val::symbol("def"), Val::symbol("a"), Val::from(99)]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::None));
        assert_eq!(vm.env.lookup(&Rc::new(Str::from("a"))), Some(Val::from(99)));
    }

    #[test]
    fn test_evaluating_symbol() {
        let mut vm = Vm::new(null_env());
        vm.env.insert(Rc::new(Str::from("a")), Val::from(12));
        assert_eq!(vm.eval_top_level(Val::symbol("a")), Ok(Val::from(12)));
        assert_eq!(
            vm.eval_top_level(Val::symbol("e")),
            Err(Error::Undeclared("e".to_string()))
        );
    }

    #[test]
    fn test_evaluating_if() {
        let mut vm = Vm::new(null_env());

        // two branches
        let expr = Val::list_from_vec(&vec![
            Val::symbol("if"),
            Val::Bool(true),
            Val::from(5),
            Val::from(10),
        ]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::from(5)));

        let expr = Val::list_from_vec(&vec![
            Val::symbol("if"),
            Val::Bool(false),
            Val::from(5),
            Val::from(10),
        ]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::from(10)));

        // Only true branch
        let expr = Val::list_from_vec(&vec![Val::symbol("if"), Val::Bool(true), Val::from(5)]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::from(5)));

        let expr = Val::list_from_vec(&vec![Val::symbol("if"), Val::Bool(false), Val::from(5)]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::None));
    }

    #[test]
    fn test_evaluating_lambda() {
        let mut vm = Vm::new(null_env());

        let expr = Val::list_from_vec(&vec![
            Val::symbol("lambda"),
            Val::from(vec![Val::symbol("x")]),
            Val::symbol("x"),
        ]);
        let closure = Val::from(Closure::new(
            None,
            vm.env.clone(),
            Formals::Fixed(vec![Rc::new(Str::from("x"))]),
            Some(Rc::new(List::new(Val::symbol("x"), None))),
        ));
        assert_eq!(vm.eval_top_level(expr), Ok(closure));
    }

    #[test]
    fn test_evaluating_closure_application() {
        let mut vm = Vm::new(null_env());

        let closure = Val::from(Closure::new(
            None,
            vm.env.clone(),
            Formals::Fixed(vec![Rc::new(Str::from("x"))]),
            Some(Rc::new(List::new(Val::symbol("x"), None))),
        ));
        let expr = Val::list_from_vec(&vec![closure, Val::from(5)]);
        assert_eq!(vm.eval_top_level(expr), Ok(Val::from(5)));
    }

    #[test]
    fn test_evaluating_procedure_application() {
        let mut vm = Vm::new(null_env());

        let proc = Val::from(Procedure::new("test-proc", |args| {
            Ok(Val::from(Vec::from(args)))
        }));
        let expr = Val::list_from_vec(&vec![proc, Val::from(5), Val::from(10), Val::from(15)]);
        assert_eq!(
            vm.eval_top_level(expr),
            Ok(Val::from(vec![Val::from(5), Val::from(10), Val::from(15)]))
        );
    }

    #[test]
    fn test_evaluating_function_call_with_symbols() {
        let mut vm = Vm::new(null_env());

        let proc = Val::from(Procedure::new("test-proc", |args| {
            Ok(Val::from(Vec::from(args)))
        }));
        vm.env.insert(Rc::new(Str::from("test-proc")), proc);
        vm.env.insert(Rc::new(Str::from("a")), Val::from(10));

        // (test-proc 5 a 15)
        let expr = Val::list_from_vec(&vec![
            Val::symbol("test-proc"),
            Val::from(5),
            Val::symbol("a"),
            Val::from(15),
        ]);
        assert_eq!(
            vm.eval_top_level(expr),
            Ok(Val::from(vec![Val::from(5), Val::from(10), Val::from(15)]))
        );
    }

    #[test]
    fn test_evaluating_forms() {
        let mut vm = Vm::new(null_env());
        let forms = vec![
            Val::from("hello"),
            Val::from(10),
            Val::keyword("hello"),
            Val::Bool(true),
        ];
        assert_eq!(vm.eval_forms(&forms), Ok(Val::Bool(true)));
    }

    #[test]
    fn test_evaluate_and_apply_macro_rules() {
        let mut vm = Vm::new(null_env());
        let forms = vec![
            // (macro-rules m [] [(m 10) 15])
            Val::list_from_vec(&vec![
                Val::symbol("macro-rules"),
                Val::symbol("m"),
                Val::from(vec![]),
                Val::from(vec![
                    Val::list_from_vec(&vec![Val::symbol("m"), Val::from(10)]),
                    Val::from(15),
                ]),
            ]),
            // (m 10)
            Val::list_from_vec(&vec![Val::symbol("m"), Val::from(10)]),
        ];
        assert_eq!(vm.eval_forms(&forms), Ok(Val::from(15)));
    }
}
