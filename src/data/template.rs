use crate::data::*;
use std::rc::Rc;

// TODO I am still trying to get the template to rename variables properly. The
// problem is that I cannot tell with my setup whether a symbol should be renamed
// or not in the expansion. Either I acciedentally rename legitimate identifiers
// like let or I do not rename a bound symbol because it is present in the environment.
// Maybe I need to be able to capture the macro environment so that I can tell what
// symbols were symbols when the macro was created. This would make it possible
// in a macro that has a let template to know that let is an identifier and that
// a binding variable is being introduced (provided it is not a capture variable).
//
// There are a number of issues still that seem to be because it is difficult to
// share information between macro, pattern, template. I think I might need to take
// another stab at creating a rule or whole macro that is all one sruct. I think it
// could still use enums for templates and patterns, but they would be used in the
// rule/syntax impl rather than having their own. It would also enable me to rename
// the construct to syntax-rules or something.
//
// Syntax rules needs to be more like a function that produces a macro transformer.
// So we would use (def name (syntax-rules name [] [rules])). This would also mean
// that we could create a transformer and save it in a let as well, though there might
// be some issues with recursion like with functions. The idea would be that when name
// is created it can capture it's env like a closure and when it is applied it can
// also use the env that it is applied in. We could also see it as a procedure
// that when evaluated produces an object that captures the new env and is then
// applied to the arguments list. However, since we currently apply things after
// evaluating everything in a funciton call list it makes more sense to just see
// the bound rules as the evaluated form and the application as separate from
// procedure application in that it does not evaluate arguments. Or maybe since
// we generally have a symbol that needs to be looked up we can look it up and if
// it is a macro we can decide to eval it and apply it and if not we can eval
// everything and then apply the proc/closure like normal.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Template {
    Atom(Val),
    Var(Rc<Str>),
    List(Vec<Template>, Option<Box<Template>>),
    Vector(Vec<Template>, Option<Box<Template>>),
}

impl Template {
    // Construct //

    pub fn new(expr: Val) -> Result<Template, Error> {
        match expr {
            Val::Symbol(s) => Ok(Template::Var(s)),
            Val::Bool(_) | Val::Char(_) | Val::Number(_) | Val::String(_) | Val::Keyword(_) => {
                Ok(Template::Atom(expr))
            }
            Val::List(ref ls) => {
                let (templates, elipse) = Template::from_collection(ls.values(), expr)?;
                Ok(Template::List(templates, elipse))
            }
            Val::Vector(ref vec) => {
                let e = expr.clone();
                let (templates, elipse) =
                    Template::from_collection(vec.borrow().values().cloned(), e)?;
                Ok(Template::Vector(templates, elipse))
            }
            _ => Err(Error::BadTemplate(expr)),
        }
    }

    fn from_collection(
        iter: impl Iterator<Item = Val>,
        expr: Val,
    ) -> Result<(Vec<Template>, Option<Box<Template>>), Error> {
        let mut pxs = Vec::new();
        let mut elipse = None;
        for val in iter {
            match val {
                Val::Symbol(s) if s.is("...") => {
                    match pxs.pop() {
                        Some(temp) => elipse = Some(Box::new(temp)),
                        _ => return Err(Error::BadTemplate(expr)),
                    };
                    break;
                }
                _ => pxs.push(Template::new(val.clone())?),
            }
        }
        Ok((pxs, elipse))
    }

    // Expand //

    pub fn expand(&self, name: Rc<Str>, captures: Environ) -> Result<Val, Error> {
        match self {
            Template::Atom(val) => Ok(val.clone()),
            Template::Var(s) => {
                //println!("{name} {s}");
                let name = Rc::new(Str::from(format!("{s}!!0").as_str()));
                match captures.lookup(&name) {
                    Some(val) => {
                        //println!("{val}");
                        Ok(val)
                    }
                    None => {
                        if *s != name && !self.reserved(s.clone()) {
                            let sym = Rc::new(Str::from(format!("{s}").as_str()));
                            Ok(Val::Symbol(sym))
                        } else {
                            Ok(Val::Symbol(s.clone()))
                        }
                    } //Ok(Val::Symbol(s.clone())),
                }
            }
            Template::List(templates, elipse) => self
                .expand_collection(name, templates, elipse, captures.clone())
                .map(|v| Val::list_from_vec(&v)),
            Template::Vector(templates, elipse) => self
                .expand_collection(name, templates, elipse, captures.clone())
                .map(Val::from),
        }
    }

    fn expand_collection(
        &self,
        name: Rc<Str>,
        templates: &[Template],
        elipse: &Option<Box<Template>>,
        captures: Environ,
    ) -> Result<Vec<Val>, Error> {
        // Expand the templates
        let mut vec = templates
            .iter()
            .map(|t| t.expand(name.clone(), captures.clone()))
            .collect::<Result<Vec<Val>, Error>>()?;

        // Expand the elipse
        if let Some(t) = elipse {
            match **t {
                Template::Var(ref s) => {
                    let mut idx = 0;
                    loop {
                        let name = Rc::new(Str::from(format!("{s}!!{idx}").as_str()));
                        match captures.lookup(&name) {
                            Some(val) => vec.push(val),
                            None => break,
                        };
                        idx += 1;
                    }
                }
                _ => {
                    // TODO this will not work with things like letrec
                    // where we do (var <literal>) ...
                    // Or possibly even with (var val) ...
                    // As mentioned in a number of places the macro needs to be
                    // re-written so that it covers more cases properly.
                    // println!("{:?}", t);
                    // println!("{:?}", captures);
                    return Err(Error::TemplateElipseNotVar);
                }
            }
        };

        Ok(vec)
    }

    // TODO this should be somewhere else
    fn reserved(&self, ident: Rc<Str>) -> bool {
        match ident.to_string().as_str() {
            "if" | "lambda" => true,
            _ => false,
        }
    }
}

// Testing ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanding_with_atoms() {
        let name = Rc::new(Str::from("name"));
        let captures = Rc::new(Env::new());

        let t = Template::Atom(Val::from(10));
        assert_eq!(t.expand(name, captures.clone()), Ok(Val::from(10)));
    }

    #[test]
    fn test_expanding_with_list() {
        let name = Rc::new(Str::from("name"));
        let captures = Rc::new(Env::new());

        let t = Template::List(
            vec![Template::Atom(Val::from(10)), Template::Atom(Val::from(99))],
            None,
        );
        assert_eq!(
            t.expand(name, captures.clone()),
            Ok(Val::list_from_vec(&vec![Val::from(10), Val::from(99)]))
        );
    }

    #[test]
    fn test_expanding_with_nested_list() {
        let name = Rc::new(Str::from("name"));
        let captures = Rc::new(Env::new());

        let t = Template::List(
            vec![
                Template::Atom(Val::from(10)),
                Template::List(vec![Template::Atom(Val::from(88))], None),
                Template::Atom(Val::from(99)),
            ],
            None,
        );
        assert_eq!(
            t.expand(name, captures.clone()),
            Ok(Val::list_from_vec(&vec![
                Val::from(10),
                Val::list_from_vec(&vec![Val::from(88)]),
                Val::from(99)
            ]))
        );
    }

    #[test]
    fn test_expanding_with_nested_list_and_vars() {
        let name = Rc::new(Str::from("f"));
        let captures = Rc::new(Env::new());
        captures.insert(Rc::new(Str::from("a!!0")), Val::from(10));
        captures.insert(Rc::new(Str::from("b!!0")), Val::from(99));

        // (f (a) b c)
        let a = Rc::new(Str::from("a"));
        let b = Rc::new(Str::from("b"));
        let c = Rc::new(Str::from("c"));
        let t = Template::List(
            vec![
                Template::Atom(Val::symbol("f")),
                Template::List(vec![Template::Var(a)], None),
                Template::Var(b),
                Template::Var(c),
            ],
            None,
        );

        // (f (10) 99 c)
        assert_eq!(
            t.expand(name, captures.clone()),
            Ok(Val::list_from_vec(&vec![
                Val::symbol("f"),
                Val::list_from_vec(&vec![Val::from(10)]),
                Val::from(99),
                Val::symbol("c"),
            ]))
        );
    }

    #[test]
    fn test_expanding_with_list_elipse_and_vars() {
        let name = Rc::new(Str::from("f"));
        let captures = Rc::new(Env::new());
        captures.insert(Rc::new(Str::from("a!!0")), Val::from(10));
        captures.insert(Rc::new(Str::from("a!!1")), Val::from(11));
        captures.insert(Rc::new(Str::from("a!!2")), Val::from(12));
        captures.insert(Rc::new(Str::from("b!!0")), Val::from(99));

        // (f (a ...) b c) where there are 3 elements in the matched expr
        let a = Rc::new(Str::from("a"));
        let b = Rc::new(Str::from("b"));
        let c = Rc::new(Str::from("c"));
        let t = Template::List(
            vec![
                Template::Atom(Val::symbol("f")),
                Template::List(vec![], Some(Box::new(Template::Var(a)))),
                Template::Var(b),
                Template::Var(c),
            ],
            None,
        );

        // (f (10 11 12) 99)
        assert_eq!(
            t.expand(name, captures.clone()),
            Ok(Val::list_from_vec(&vec![
                Val::symbol("f"),
                Val::list_from_vec(&vec![Val::from(10), Val::from(11), Val::from(12)]),
                Val::from(99),
                Val::symbol("c"),
            ]))
        );
    }

    #[test]
    fn test_expanding_with_vector_elipse_and_vars() {
        let name = Rc::new(Str::from("name"));
        let captures = Rc::new(Env::new());
        captures.insert(Rc::new(Str::from("a!!0")), Val::from(10));
        captures.insert(Rc::new(Str::from("a!!1")), Val::from(11));
        captures.insert(Rc::new(Str::from("a!!2")), Val::from(12));
        captures.insert(Rc::new(Str::from("b!!0")), Val::from(99));

        // [f [a ...] b c] where there are 3 elements in the matched expr
        let a = Rc::new(Str::from("a"));
        let b = Rc::new(Str::from("b"));
        let c = Rc::new(Str::from("c"));
        let t = Template::Vector(
            vec![
                Template::Atom(Val::symbol("f")),
                Template::Vector(vec![], Some(Box::new(Template::Var(a)))),
                Template::Var(b),
                Template::Var(c),
            ],
            None,
        );

        // [f [10 11 12] 99]
        assert_eq!(
            t.expand(name, captures.clone()),
            Ok(Val::from(vec![
                Val::symbol("f"),
                Val::from(vec![Val::from(10), Val::from(11), Val::from(12)]),
                Val::from(99),
                Val::symbol("c"),
            ]))
        );
    }
}
