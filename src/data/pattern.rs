use crate::data::*;
use std::rc::Rc;

// TODO I have the expected functionality for everything except the scheme
// way of handing literals and symbols when a symbol is defined outside the
// macro use. However, I have not addressed all of the possible errors.
// I.e. it is an error to use an elipse anywhere but at the end of a list or vector.
//
// TODO I beleive that most of the current implementation for correct patterns is
// complete, but there is no testing for errors.
//
// TODO for a full implementation either Maps should have the same behaviour as
// lists and vectors or they should match as literals. The first is the best, but
// I wanted to get it working for now ignoring maps, but could do the second in
// the interim.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Atom(Rc<Str>, Val),
    Var(Rc<Str>),
    List(Vec<Pattern>, Option<Box<Pattern>>),
    Vector(Vec<Pattern>, Option<Box<Pattern>>),
}

impl Pattern {
    // Construct //

    pub fn new(name: Rc<Str>, syms: &[Rc<Str>], expr: Val) -> Result<Pattern, Error> {
        match expr {
            Val::Symbol(s) => {
                if s == name || syms.contains(&s) {
                    Ok(Pattern::Atom(name, Val::symbol(s.to_string().as_str())))
                } else {
                    Ok(Pattern::Var(s))
                }
            }
            Val::Bool(_) | Val::Char(_) | Val::Number(_) | Val::String(_) | Val::Keyword(_) => {
                Ok(Pattern::Atom(name, expr))
            }
            Val::List(ref ls) => {
                let (patterns, elipse) = Pattern::from_collection(ls.values(), name, syms, expr)?;
                Ok(Pattern::List(patterns, elipse))
            }
            Val::Vector(ref vec) => {
                let e = expr.clone();
                let (patterns, elipse) =
                    Pattern::from_collection(vec.borrow().values().cloned(), name, syms, e)?;
                Ok(Pattern::Vector(patterns, elipse))
            }
            _ => Err(Error::BadPattern(expr)),
        }
    }

    fn from_collection(
        iter: impl Iterator<Item = Val>,
        name: Rc<Str>,
        syms: &[Rc<Str>],
        expr: Val,
    ) -> Result<(Vec<Pattern>, Option<Box<Pattern>>), Error> {
        let mut pxs = vec![];
        let mut elipse = None;
        for val in iter {
            match val {
                Val::Symbol(s) if s.is("...") => {
                    match pxs.pop() {
                        Some(pat) => elipse = Some(Box::new(pat)),
                        _ => return Err(Error::BadPattern(expr)),
                    };
                    break;
                }
                _ => pxs.push(Pattern::new(name.clone(), syms, val.clone())?),
            }
        }
        Ok((pxs, elipse))
    }

    // Matching //

    pub fn matches(&self, expr: Val, env: Environ) -> Option<Environ> {
        self.matches_rec(expr, env, Rc::new(Env::new()), 0)
    }

    fn matches_rec(
        &self,
        expr: Val,
        env: Environ,
        captures: Environ,
        idx: usize,
    ) -> Option<Environ> {
        match self {
            Pattern::Atom(name, val) => {
                if let Val::Symbol(s) = val {
                    if s != name && env.lookup(&s).is_some() {
                        return None;
                    }
                }
                if *val == expr {
                    Some(captures)
                } else {
                    None
                }
            }
            Pattern::Var(s) => {
                let name = Rc::new(Str::from(format!("{s}!!{idx}").as_str()));
                captures.insert(name, expr);
                Some(captures)
            }
            Pattern::List(patterns, elipse) => match expr {
                Val::List(ls) => self.match_collection(
                    ls.values().collect(),
                    patterns,
                    elipse,
                    idx,
                    captures.clone(),
                    env.clone(),
                ),
                _ => None,
            },
            Pattern::Vector(patterns, elipse) => match expr {
                Val::Vector(vals) => self.match_collection(
                    vals.borrow().values().cloned().collect(),
                    patterns,
                    elipse,
                    idx,
                    captures.clone(),
                    env.clone(),
                ),
                _ => None,
            },
        }
    }

    fn match_collection(
        &self,
        vals: Vec<Val>,
        patterns: &[Pattern],
        elipse: &Option<Box<Pattern>>,
        idx: usize,
        captures: Environ,
        env: Environ,
    ) -> Option<Environ> {
        println!("{:?}", vals);
        let mut captures = captures;
        // match patterns
        for (i, p) in patterns.iter().enumerate() {
            captures = match p.matches_rec(vals[i].clone(), env.clone(), captures, idx) {
                Some(captures) => captures,
                None => return None,
            };
        }
        // match elipse
        if patterns.len() < vals.len() {
            match elipse {
                Some(pat) => {
                    for (i, val) in vals[patterns.len()..].iter().enumerate() {
                        captures = match pat.matches_rec(val.clone(), env.clone(), captures, i) {
                            Some(captures) => captures,
                            None => return None,
                        }
                    }
                }
                None => return None,
            };
        }
        // return captures
        Some(captures)
    }
}

// Testing ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    // Creation //
    #[test]
    fn test_creating_atoms() {
        let name = Rc::new(Str::from("name"));
        let syms = vec![Rc::new(Str::from("sym"))];
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::symbol("sym")),
            Ok(Pattern::Atom(name.clone(), Val::symbol("sym")))
        );
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::from('A')),
            Ok(Pattern::Atom(name.clone(), Val::from('A')))
        );
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::Bool(true)),
            Ok(Pattern::Atom(name.clone(), Val::Bool(true)))
        );
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::from(18)),
            Ok(Pattern::Atom(name.clone(), Val::from(18)))
        );
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::from("hello, world!")),
            Ok(Pattern::Atom(name.clone(), Val::from("hello, world!")))
        );
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::keyword(":hello")),
            Ok(Pattern::Atom(name.clone(), Val::keyword(":hello")))
        );
    }

    #[test]
    fn test_creating_var() {
        let name = Rc::new(Str::from("name"));
        let syms = vec![];
        assert_eq!(
            Pattern::new(name.clone(), &syms, Val::symbol("let")),
            Ok(Pattern::Var(Rc::new(Str::from("let"))))
        );
    }

    #[test]
    fn test_creating_list() {
        let name = Rc::new(Str::from("cons"));
        let syms = vec![Rc::new(Str::from("list"))];

        // (cons 89 (list 2 10))
        let expr = Val::list_from_vec(&vec![
            Val::symbol("cons"),
            Val::from(89),
            Val::list_from_vec(&vec![Val::symbol("list"), Val::from(2), Val::from(10)]),
        ]);

        // Pattern
        let pat = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("cons")),
                Pattern::Atom(name.clone(), Val::from(89)),
                Pattern::List(
                    vec![
                        Pattern::Atom(name.clone(), Val::symbol("list")),
                        Pattern::Atom(name.clone(), Val::from(2)),
                        Pattern::Atom(name.clone(), Val::from(10)),
                    ],
                    None,
                ),
            ],
            None,
        );

        assert_eq!(Pattern::new(name.clone(), &syms, expr), Ok(pat));
    }

    #[test]
    fn test_creating_vectors() {
        let name = Rc::new(Str::from("name"));
        let syms = vec![Rc::new(Str::from("push"))];

        // [push 89 [2 10]]
        let expr = Val::from(vec![
            Val::symbol("push"),
            Val::from(89),
            Val::from(vec![Val::from(2), Val::from(10)]),
        ]);

        // Pattern
        let pat = Pattern::Vector(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("push")),
                Pattern::Atom(name.clone(), Val::from(89)),
                Pattern::Vector(
                    vec![
                        Pattern::Atom(name.clone(), Val::from(2)),
                        Pattern::Atom(name.clone(), Val::from(10)),
                    ],
                    None,
                ),
            ],
            None,
        );

        assert_eq!(Pattern::new(name.clone(), &syms, expr), Ok(pat));
    }

    #[test]
    fn test_creating_vec_with_elipse() {
        let name = Rc::new(Str::from("name"));
        let syms = vec![];

        // [1 2 ...]
        let expr = Val::from(vec![Val::from(1), Val::from(2), Val::symbol("...")]);
        let pat = Pattern::Vector(
            vec![Pattern::Atom(name.clone(), Val::from(1))],
            Some(Box::new(Pattern::Atom(name.clone(), Val::from(2)))),
        );

        assert_eq!(Pattern::new(name.clone(), &syms, expr), Ok(pat));
    }

    #[test]
    fn test_creating_nested_lists_with_elipses() {
        let name = Rc::new(Str::from("let"));
        let syms = vec![];

        // (let ((var val) ...) body ...)
        let expr = Val::list_from_vec(&vec![
            Val::symbol("let"),
            Val::list_from_vec(&vec![
                Val::list_from_vec(&vec![Val::symbol("var"), Val::symbol("val")]),
                Val::symbol("..."),
            ]),
            Val::symbol("body"),
            Val::symbol("..."),
        ]);

        // Pattern
        let pat = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("let")),
                Pattern::List(
                    vec![],
                    Some(Box::new(Pattern::List(
                        vec![
                            Pattern::Var(Rc::new(Str::from("var"))),
                            Pattern::Var(Rc::new(Str::from("val"))),
                        ],
                        None,
                    ))),
                ),
            ],
            Some(Box::new(Pattern::Var(Rc::new(Str::from("body"))))),
        );

        assert_eq!(Pattern::new(name.clone(), &syms, expr), Ok(pat));
    }

    // Matching //

    #[test]
    fn test_matching_atoms() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let p = Pattern::Atom(name.clone(), Val::from(10));
        assert_eq!(
            p.matches_rec(Val::from(10), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        assert_eq!(
            p.matches_rec(Val::from(30), env.clone(), captures.clone(), 0),
            None
        );

        let p = Pattern::Atom(name.clone(), Val::from("hello, world!"));
        assert_eq!(
            p.matches_rec(Val::from("hello, world!"), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        assert_eq!(
            p.matches_rec(Val::from("not it"), env.clone(), captures.clone(), 0),
            None
        );
    }

    #[test]
    fn test_matching_atom_when_sym_is_in_env() {
        let name = Rc::new(Str::from("name"));
        // Atoms that are symbols should not match if an identifier is in the
        // enclosing environment. The example in R5RS is for cond inside a let where
        // => is bound in let. The match for cond treats => as an identifier rather
        // than as the => used in cond clauses. If => was in the syms list when
        // creating the pattern the pattern created for it would be an Atom instead
        // of a Var. However, when we match against it we should not automatically
        // assume it is a match if the pattern matches a symbol in the expr. It only
        // matches if the identifier is not bound in the enclosing env. Not completely
        // sure I interpreted this properly, but this will be the behaviour I
        // implement.
        // NOTE if symbol is the name it should still match, as it should always
        // be in the env.
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let p = Pattern::Atom(name.clone(), Val::symbol("=>"));
        assert_eq!(
            p.matches_rec(Val::symbol("=>"), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        env.insert(Rc::new(Str::from("=>")), Val::from(99));
        assert_eq!(
            p.matches_rec(Val::symbol("=>"), env.clone(), captures.clone(), 0),
            None
        );
    }

    #[test]
    fn test_matching_with_var() {
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let var = Rc::new(Str::from("name"));
        let fixed_var = Rc::new(Str::from("name!!0"));
        let p = Pattern::Var(var.clone());
        assert_eq!(
            p.matches_rec(Val::from(10), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        assert_eq!(captures.lookup(&fixed_var), Some(Val::from(10)));
    }

    #[test]
    fn test_matching_list() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let expr = Val::list_from_vec(&vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
        ]);
        let p = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(10)),
                Pattern::Atom(name.clone(), Val::from("hello, world!")),
            ],
            None,
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );

        let p = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(99)),
                Pattern::Atom(name.clone(), Val::from("hello")),
            ],
            None,
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            None
        );
    }

    #[test]
    fn test_matching_vector() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let expr = Val::from(vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
        ]);
        let p = Pattern::Vector(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(10)),
                Pattern::Atom(name.clone(), Val::from("hello, world!")),
            ],
            None,
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
    }

    #[test]
    fn test_matching_nested_list_with_vars() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let list = Val::list_from_vec(&vec![Val::symbol("+"), Val::symbol("x"), Val::symbol("y")]);
        let expr = Val::list_from_vec(&vec![
            Val::symbol("lambda"),
            Val::list_from_vec(&vec![Val::symbol("x"), Val::symbol("y")]),
            list.clone(),
        ]);

        let a = Rc::new(Str::from("a"));
        let b = Rc::new(Str::from("b"));
        let body = Rc::new(Str::from("body"));
        let p = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("lambda")),
                Pattern::List(vec![Pattern::Var(a.clone()), Pattern::Var(b.clone())], None),
                Pattern::Var(body.clone()),
            ],
            None,
        );

        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        assert_eq!(
            captures.lookup(&Rc::new(Str::from("a!!0"))),
            Some(Val::symbol("x"))
        );
        assert_eq!(
            captures.lookup(&Rc::new(Str::from("b!!0"))),
            Some(Val::symbol("y"))
        );
        assert_eq!(captures.lookup(&Rc::new(Str::from("body!!0"))), Some(list));
    }

    #[test]
    fn test_matching_elipse_list() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let expr = Val::list_from_vec(&vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
        ]);
        let p = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(10)),
            ],
            Some(Box::new(Pattern::Atom(
                name.clone(),
                Val::from("hello, world!"),
            ))),
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );

        let expr = Val::list_from_vec(&vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
        ]);
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );

        let expr = Val::list_from_vec(&vec![Val::symbol("name"), Val::from(10)]);
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
    }

    #[test]
    fn test_matching_elipse_vector() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let expr = Val::from(vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
        ]);
        let p = Pattern::Vector(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(10)),
            ],
            Some(Box::new(Pattern::Atom(
                name.clone(),
                Val::from("hello, world!"),
            ))),
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );

        let expr = Val::from(vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
            Val::from("hello, world!"),
        ]);
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );

        let expr = Val::from(vec![Val::symbol("name"), Val::from(10)]);
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
    }

    #[test]
    fn test_matching_elipse_list_with_var() {
        let name = Rc::new(Str::from("name"));
        let env = Rc::new(Env::new());
        let captures = Rc::new(Env::new());

        let expr = Val::list_from_vec(&vec![
            Val::symbol("name"),
            Val::from(10),
            Val::from(77),
            Val::from(88),
            Val::from(99),
        ]);
        let var = Rc::new(Str::from("x"));
        let p = Pattern::List(
            vec![
                Pattern::Atom(name.clone(), Val::symbol("name")),
                Pattern::Atom(name.clone(), Val::from(10)),
            ],
            Some(Box::new(Pattern::Var(var))),
        );
        assert_eq!(
            p.matches_rec(expr.clone(), env.clone(), captures.clone(), 0),
            Some(captures.clone())
        );
        assert_eq!(
            captures.lookup(&Rc::new(Str::from("x!!0"))),
            Some(Val::from(77))
        );
        assert_eq!(
            captures.lookup(&Rc::new(Str::from("x!!1"))),
            Some(Val::from(88))
        );
        assert_eq!(
            captures.lookup(&Rc::new(Str::from("x!!2"))),
            Some(Val::from(99))
        );
    }
}
