// Expose all functions and structs that will be used by compiled code if
// it is possible, so that we can just import runtime::* in the compiled files

// This works with a runtime environment, the biggest issue is lookup times for
// every single variable. The next example makes a module into a struct and
// saves all definitions in a field with an rc<refcell<>> so they are easy
// to pass around and can be mutated. This way the worst thing about a
// variable access is an Rc clone and following pointers to the data. The
// runtime env would have all of that but also have potentially many hash table
// lookups.
pub use crate::data::{Error, Lambda, Str, Val};

use std::cell::RefCell;
use std::rc::Rc;

/*
pub fn lambda(
    name: Option<&str>,
    env: Environment,
    params: Vec<&'static str>,
    func: fn(env: Environment) -> Result<Val, Error>,
) -> Val {
    let n = match name {
        Some(s) => Some(Str::from(s)),
        None => None,
    };

    Val::from(Closure::new(n, env, params, func))
}

pub fn apply(val: Val, args: Vec<Val>) -> Result<Val, Error> {
    match val {
        Val::Closure(c) => {
            let env = Rc::new(Env::new_scope(c.env.clone()));
            for (i, p) in c.params.iter().enumerate() {
                c.env.put(p, args[i].clone())
            }
            let f = c.func;
            f(env)
        }
        Val::Procedure(p) => {
            let f = p.func;
            f(args)
        }
        _ => Err(Error::NotAProcedure(val)),
    }
}

// Testing

pub fn module(env: Environment) -> Result<Environment, Error> {
    // (define a 5)
    env.put("a", Val::from(5));

    // (define (f x) (+ x 1))
    env.put(
        "f",
        lambda(Some("f"), env.clone(), vec!["x"], |env| {
            apply(env.get("+")?, vec![env.get("x")?, Val::from(1)])
        }),
    );
    Ok(env)
}
*/

/*
 * The above is essentially the easiest way to generate modules that can be loaded
 * and have their identifiers added to an environment. I was going to pass integers
 * to the env get and set as a way of making the env interacitons faster, but it
 * comes with a difficulty. The env being passed into the module must be known
 * at compile time. This works fine if a program is always made of modules
 * and compiled all at once, but it prevents the compiling of modules that are
 * not part of a program for import elsewhere. The above solves this as each symbol
 * is looked up at runtime and therfore the module can just be loaded provided
 * there is an environment to pass to it. An interesting side effect of this is
 * that it would be possible to load a module inside of a scope, like a let, and
 * not have the bindings affect the outer scopes.
 *
 * One optimisation that I though about for other parts of the code is to find
 * a way to store the value obtained by get in a local rust var at the beginning
 * of a scope. That way it can be cloned around where needed, but we do not have
 * to hash and lookup the value everytime it is needed. This would probably be
 * possible everywhere except when creating lambda bodies. Many places if the name
 * is only used once it would not make any difference, but if a value is defined
 * at the top of the mod and refered to in many other places it could save some
 * lookups.
 *
 * I still think the ideal way to compile would be to actually make definitions
 * rust code. The reason I did not do this previously is because it is not
 * possible to execute arbitrairy expressions or even set non-const vars
 * in a file, but it is not possible to create functions inside another function
 * in the same way we might want. Possibly we could use fn as a way around the
 * second problem, but then again all of our definitions are inside a function
 * and difficult/impossible to export. The other way would be to solve the first
 * problems in some way. The definitions could be replaced with a function that
 * when called yields the value that would have been saved. If the function were
 * somehow a promise it could be done in a way that was useful, but I am not sure
 * that is possible, unless perhaps we find a way to initialize the memory at the
 * start of the program/module. I suppose that a module could have it's definitions
 * and also an init function to do that, but there would be some issues here still
 * I think.
 *
 * I think the answer to these issues is using classes for modules. I.e. a module
 * is a struct, it's defined vars are fields, and it's functions are methods. All
 * arbirairy expressions and non-function definitions can be executed in the
 * constructor. When a class wants to require/load a module we store it as a field
 * in their set of fields and initialize it in the constructor. The only
 * difficulty I see here is making sure not to double initialize a module, but
 * instead find a way to load it once and pass a cloned version or something to
 * structs that want to initialize it again. But that would suggest initializing it
 * in a main somewhere and passing it to module init functions so that it can
 * keep track of whether it has been initialized or not. However, even without that
 * ability it might be the best way of setting up compiled code. All variables would
 * have to be inside rc<refcell<>> to make them setable, but it is going to be less
 * computation than working with the runtime environment and hash tables.
 *
 * The only real serious difficulty here is being able to setup closures properly.
 * I beleive that I had a way to create new local variables just outside of the functions
 * so that they could be cloned into a var and that var could be moved into the
 * closure so that the closure could hold a reference properly, but it will not
 * be possible to save an entire environment if there is no environment at runtime.
 */

/*
 * I think the below will work quite nicely for just regular scheme code, though
 * it will need some helpers or work to make it nice to use with rust. Obviously,
 * many problems are solved and we do not need an environment, but now a module
 * is a class with Var fields. It is not possible to just import it like a
 * rust module and call functions or access members. Though module.field.get
 * will get a value and module.field.apply(vec![args ...]) will allow calling
 * functions.
 *
 * Stdlib functions will have to be autoloaded in everymodule. We will still need
 * a compile time environment to at the very least keep track of which variable
 * should be used for a symbol if it has been bound in multiple scopes. The stdlib
 * can have all of the functions in the various files in the project, but will need
 * a module that looks like this one to define the Val::Procedure objects that
 * will be in the stdlib.
 *
 * There also should be some way to prevent changing some identifiers with set!.
 * For example, keywords and derived expressions cannot be set as they are not
 * in the environment. They can be bound and re-bound in a scope and then set,
 * but it would be an error to set! them. It should also be an error to try and
 * call set on a stdlib variable. It would also be nice to setup somekind of
 * define-const that can be used to define a variable that will not be mutable
 * in a module. The most useful thing would be for constant values like Pi etc.
 * While it is easy to say that you just should not modify a variable, loading
 * something and setting it could cause problems for other modules that rely on
 * it. I.e. if you load my module and I set Pi to 0.1 it will mess up all computation
 * that use it. I suppose I can see moments where it would be nice to be able to
 * alter a constant value so that you could alter the behaviour of other functions
 * without having to rewrite them, it seems like the accidental changes would
 * be more common than legitimate uses of this. So providing a module writer
 * with the ability to protect a constant against set! would be useful. There
 * are things like mocking with testing that could benefit from redefining
 * variables at the module level, but languages like clojure wrap this in some
 * kind of binding construct to ensure that the changes are only visible in
 * a certain context and are fixed afterwards. I think for now I will just leave
 * them visible, but I will think about it as I update things.
 *
 * Another way that the above could be setup is to have some kind of visibility
 * specifier. The module writer could set an internal variable that is not
 * accessible to the user and then expose the variable with a copy of the private
 * varaible. I.e. you have __pi and PI and all internal uses of pi are with the
 * __pi and users can access PI. If they set! PI it will only affect their code
 * and not the module's code.
 *
 * Another need is name mangling for the compiler. The simple need is just to
 * ensure that the invalid symbols are not used in rust identifiers. The complex
 * need is that if you define a function in scheme that you want to call from
 * rust it should be sensibly named when mangled. I thing that perhaps all symbols
 * can just be replaced like so * => _star, ? => _qmrk, ! => _excl, $ => _dolr,
 * # => _hash, @ => _at, % => _perc, ^ => _caret, & => _amp, - => _, = => _eq,
 * + => _plus, / => _frsl, \ => _bksl, . => _dot, < => _less, > => _grtr, ~ => _tild
 * Any others are reserved for other forms ()[]{},"'`
 * The - => _ could cause some confusion, but I think it is the best way to
 * approach it and just be clear that - and _ are the same in identifiers. The
 * best approach for wanting to write functions that could be called from rust
 * would just be to make identifiers that are rust compliant, though using the
 * nicer (imo) - to separate words. Since many things will be checked at compile
 * time the mangling should not affect errors too much, and where it might we
 * can save strings, like with proc names, to be displayed for the proper name.
 * I.e. if I write stdlib procedures their errors can contain their names. Any,
 * runtime errors that are in a closure can use its name if it has one etc.
 *
 * Module namespacing is important too. I don't think that for now I will setup
 * any default namespacing. However, I think that a module could have a load or
 * an import specifier. These would be used at compile time to decide how to
 * refer to the variables. With load we would use the compile time env to
 * lookup the scope of each variable and use the module name if it was loaded
 * after other modules or the stdlib. For import we could require :: to scope
 * the variables to the specific module either with an extra prefix being
 * specified or using the filename. I.e. (import "some/file/path.scm" abc) with
 * all calls as (abc::some-var). Using :: sort of makes sense as the scoping
 * operator, but since it will be replaced with a method call on a module instead
 * of being used as rust code it is possible it is better to use / like other
 * schemes and lisps. I suppose that the reason to use :: is that if rust interop
 * is made to work well then you would use the :: for calls to rust code as we
 * would not want to mangle those names or change them in any way. But calling
 * rust from scheme will require a bit more work as we need to know when we
 * are calling rust and have specific functions for transforming variables
 * to rust values for the functions. I.e. (::crate::mod::func (as-i64 val)) we
 * can check some of this at compile time, but not rust types, they would have
 * to be checked with cargo and rust.
 */

/*
 * (load other-mod)
 *
 * (define a 5)
 *
 * (define b 6)
 *
 * (define (f x) (+ x b))
 *
 * (f (other-mod::g(a))
 *
 */

// Temp test mod to see how imports would work //

// We can use a static mutex to keep a module from being initialized more than
// once. If the mutex is none then we create and initialize the module and set
// the value in the mutex. If it is Some then we can just clone and return the
// module object. The most important thing here is that any variables in modules
// are inside Rc and RefCell so that cloneing is simple and they can be get and
// set from anywhere the module is loaded.
use std::sync::Mutex;

static mut IS_INIT: Mutex<Option<OtherMod>> = Mutex::new(None);

#[derive(Clone)]
pub struct OtherMod {
    pub g: Var,
}

impl OtherMod {
    pub fn load() -> Result<OtherMod, Error> {
        unsafe {
            if let Ok(Some(m)) = IS_INIT.get_mut() {
                Ok(m.clone())
            } else {
                let m = OtherMod::init()?;
                *IS_INIT.get_mut().unwrap() = Some(m.clone());
                Ok(m)
            }
        }
    }

    fn init() -> Result<OtherMod, Error> {
        Ok(OtherMod {
            g: Var::new(Val::Empty), // doesn't matter for this test
        })
    }
}

// A proper module //

static mut MOD_IS_INIT: Mutex<Option<Module>> = Mutex::new(None);

#[derive(Clone)]
pub struct Module {
    pub other_mod: OtherMod,
    pub a: Var,
    pub b: Var,
    pub f: Var,
}

impl Module {
    // Would a load change if we had an (import "file" mod-name)? or would
    // that just be a thing the compiler tracked to decide what var names to
    // use for compiled code.
    pub fn load() -> Result<Module, Error> {
        // TODO can we do this in a safe way?
        unsafe {
            if let Ok(Some(m)) = MOD_IS_INIT.get_mut() {
                Ok(m.clone())
            } else {
                let m = Module::init()?;
                *MOD_IS_INIT.get_mut().unwrap() = Some(m.clone());
                Ok(m)
            }
        }
    }

    fn init() -> Result<Module, Error> {
        let other_mod = OtherMod::load()?;
        let a = Var::new(Val::from(5));
        let b = Var::new(Val::from(6));
        // might want to add other types to the lambda, probably make it the same
        // as closure without the env.
        let f = Var::new(Val::from(Lambda::new("f", {
            // captured var
            let b = b.clone();
            // closure in an rc
            Rc::new(move |args: &[Val]| sum(&[args[0].clone(), args[1].clone(), b.get()]))
        })));

        f.apply(&[other_mod.g.apply(&[a.get()])?])?;

        Ok(Module {
            other_mod: other_mod,
            a: a,
            b: b,
            f: f,
        })
    }
}

// just to make the above satisfy checks
pub fn sum(_args: &[Val]) -> Result<Val, Error> {
    Ok(Val::Empty)
}

// Possible box for values //

#[derive(Clone, Debug)]
pub struct Var {
    data: Rc<RefCell<Val>>,
}

impl Var {
    pub fn new(val: Val) -> Var {
        Var {
            data: Rc::new(RefCell::new(val)),
        }
    }

    pub fn set(&self, val: Val) {
        self.data.replace(val);
    }

    pub fn get(&self) -> Val {
        self.data.borrow().clone()
    }

    pub fn apply(&self, args: &[Val]) -> Result<Val, Error> {
        match self.data.borrow().clone() {
            Val::Lambda(lam) => lam.call(args),
            // missing other types
            val => Err(Error::NotAProcedure(val)),
        }
    }
}
