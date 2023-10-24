use crate::data::*;
use std::rc::Rc;

// TODO these are somewhat tested to ensure that general behaviour is implemented.
// However, errors are not all covered and some things that should be tested in
// template or pattern are covered by tests in here, but not in unit tests. I also
// think that the functionality I want if macros are first class and expanded at
// interpretation time. If they are fully expanded at read time then I will have
// problems with naming.

// Macro Rules ////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    pattern: Pattern,
    template: Template,
}

impl Rule {
    pub fn from_rule_expr(name: Rc<Str>, expr: Val, syms: &[Rc<Str>]) -> Result<Rule, Error> {
        let (p, t) = match expr {
            Val::Vector(ref vec) => (
                vec.borrow().get(0).ok_or(Error::BadRule(expr.clone()))?,
                vec.borrow().get(1).ok_or(Error::BadRule(expr.clone()))?,
            ),
            _ => return Err(Error::BadRule(expr)),
        };

        // NOTE the macro must always be in the form of a function call with a list
        if let Val::List(ref ls) = p {
            if let Val::Symbol(s) = ls.first().ok_or(Error::BadRule(expr.clone()))? {
                if s != name {
                    return Err(Error::BadPattern(p));
                }
            }
        } else {
            return Err(Error::BadPattern(p));
        };

        Ok(Self {
            pattern: Pattern::new(name, syms, p)?,
            template: Template::new(t)?,
        })
    }
}

// Macro //////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Macro {
    name: Rc<Str>,
    rules: Vec<Rule>,
}

impl Macro {
    pub fn new(name: Rc<Str>, syms: &[Rc<Str>], rule_exprs: Rc<List>) -> Result<Self, Error> {
        let mut full_syms = vec![name.clone()];
        for s in syms {
            full_syms.push(s.clone())
        }

        let mut rules = Vec::new();
        for expr in rule_exprs.values() {
            rules.push(Rule::from_rule_expr(name.clone(), expr, &full_syms)?);
        }

        Ok(Self { name, rules })
    }

    pub fn expand(&self, expr: Val, env: Environ) -> Result<Val, Error> {
        for rule in self.rules.iter() {
            if let Some(captures) = rule.pattern.matches(expr.clone(), env.clone()) {
                return rule.template.expand(self.name.clone(), captures);
            }
        }
        Err(Error::NoMacroMatch(self.name.to_string()))
    }
}

// Representaitons //

impl DisplayRep for Macro {
    fn to_display(&self) -> String {
        format!("#<macro {}>", self.name)
    }
}

impl ExternalRep for Macro {
    fn to_external(&self) -> String {
        self.to_display()
    }
}

impl std::fmt::Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Macro{{ {} }}", self.to_external())
    }
}

// Testing ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pattern_with_literals() {
        let env = Rc::new(Env::new());
        let name = Rc::new(Str::from("f"));
        // just 1 rule
        // ([(f 10 11)
        //   (if #t 9 10)])
        let expr = Val::list_from_vec(&vec![Val::symbol("f"), Val::from(10), Val::from(11)]);
        let result = Val::list_from_vec(&vec![
            Val::symbol("if"),
            Val::Bool(true),
            Val::from(9),
            Val::from(10),
        ]);
        let rules = List::from_vec(&vec![Val::from(vec![expr.clone(), result.clone()])]).unwrap();
        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(mac.expand(expr, env.clone()), Ok(result));
    }

    #[test]
    fn test_multi_pattern_with_literals() {
        let env = Rc::new(Env::new());
        let name = Rc::new(Str::from("f"));
        // just 1 rule
        // ([(f 44 90)
        //   (if #t 9 10)]
        //  [(f 10 11)
        //   (if #f 9 10)])
        let pat1 = Val::list_from_vec(&vec![Val::symbol("f"), Val::from(44), Val::from(90)]);
        let temp1 = Val::list_from_vec(&vec![
            Val::symbol("if"),
            Val::Bool(true),
            Val::from(9),
            Val::from(10),
        ]);
        let pat2 = Val::list_from_vec(&vec![Val::symbol("f"), Val::from(10), Val::from(11)]);
        let temp2 = Val::list_from_vec(&vec![
            Val::symbol("if"),
            Val::Bool(false),
            Val::from(9),
            Val::from(10),
        ]);
        let rules = List::from_vec(&vec![
            Val::from(vec![pat1.clone(), temp1.clone()]),
            Val::from(vec![pat2.clone(), temp2.clone()]),
        ])
        .unwrap();
        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(mac.expand(pat2, env.clone()), Ok(temp2));

        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(mac.expand(pat1, env.clone()), Ok(temp1));

        let pat3 = Val::list_from_vec(&vec![Val::symbol("u"), Val::from(10), Val::from(11)]);
        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(
            mac.expand(pat3, env.clone()),
            Err(Error::NoMacroMatch("f".to_string()))
        );
    }

    #[test]
    fn test_let_to_lambda() {
        let env = Rc::new(Env::new());
        let name = Rc::new(Str::from("let"));

        // (let ((var val) ...) body ...)
        let pat = Val::list_from_vec(&vec![
            Val::symbol("let"),
            Val::list_from_vec(&vec![
                Val::list_from_vec(&vec![Val::symbol("var"), Val::symbol("val")]),
                Val::symbol("..."),
            ]),
            Val::symbol("body"),
            Val::symbol("..."),
        ]);
        // ((lambda (var ...) body ...) val ...)
        let temp = Val::list_from_vec(&vec![
            Val::list_from_vec(&vec![
                Val::symbol("lambda"),
                Val::list_from_vec(&vec![Val::symbol("var"), Val::symbol("...")]),
                Val::symbol("body"),
                Val::symbol("..."),
            ]),
            Val::symbol("val"),
            Val::symbol("..."),
        ]);
        let rules = List::from_vec(&vec![Val::from(vec![pat.clone(), temp.clone()])]).unwrap();

        // (let ((a 5) (b 6)) (set! a 10) (+ a b))
        let expr = Val::list_from_vec(&vec![
            Val::symbol("let"),
            Val::list_from_vec(&vec![
                Val::list_from_vec(&vec![Val::symbol("a"), Val::from(5)]),
                Val::list_from_vec(&vec![Val::symbol("b"), Val::from(6)]),
            ]),
            Val::list_from_vec(&vec![Val::symbol("set!"), Val::symbol("a"), Val::from(10)]),
            Val::list_from_vec(&vec![Val::symbol("+"), Val::symbol("a"), Val::symbol("b")]),
        ]);

        // ((lambda (a b) (set! a 10) (+ a b)) 5 6)
        let result = Val::list_from_vec(&vec![
            Val::list_from_vec(&vec![
                Val::symbol("lambda"),
                Val::list_from_vec(&vec![Val::symbol("a"), Val::symbol("b")]),
                Val::list_from_vec(&vec![Val::symbol("set!"), Val::symbol("a"), Val::from(10)]),
                Val::list_from_vec(&vec![Val::symbol("+"), Val::symbol("a"), Val::symbol("b")]),
            ]),
            Val::from(5),
            Val::from(6),
        ]);

        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(mac.expand(expr, env.clone()), Ok(result));
    }

    #[test]
    fn test_nested_lists_and_vecs_without_elipse() {
        let env = Rc::new(Env::new());
        let name = Rc::new(Str::from("let*"));
        // ([(let* [(var val)] body)
        //   ((lambda [var] body) val)])
        let pat = Val::list_from_vec(&vec![
            Val::symbol("let*"),
            Val::from(vec![Val::list_from_vec(&vec![
                Val::symbol("var"),
                Val::symbol("val"),
            ])]),
            Val::symbol("body"),
        ]);
        let temp = Val::list_from_vec(&vec![
            Val::list_from_vec(&vec![
                Val::symbol("lambda"),
                Val::from(vec![Val::symbol("var")]),
                Val::symbol("body"),
            ]),
            Val::symbol("val"),
        ]);
        let rules = List::from_vec(&vec![Val::from(vec![pat.clone(), temp.clone()])]).unwrap();

        // (let* [(a 5)] a)
        let expr = Val::list_from_vec(&vec![
            Val::symbol("let*"),
            Val::from(vec![Val::list_from_vec(&vec![
                Val::symbol("a"),
                Val::from(5),
            ])]),
            Val::symbol("a"),
        ]);
        // ((lambda [a] a) 5)
        let result = Val::list_from_vec(&vec![
            Val::list_from_vec(&vec![
                Val::symbol("lambda"),
                Val::from(vec![Val::symbol("a")]),
                Val::symbol("a"),
            ]),
            Val::from(5),
        ]);

        let mac = Macro::new(name.clone(), &vec![], Rc::new(rules.clone())).unwrap();
        assert_eq!(mac.expand(expr, env.clone()), Ok(result));
    }
}
