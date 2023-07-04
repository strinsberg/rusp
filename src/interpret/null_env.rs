use crate::data::*;
use crate::procedures;
use std::rc::Rc;

pub fn null_env() -> Environ {
    // Todo add builtin procedure bindings
    let env = Env::new();
    env.insert_all(&vec![
        // lists
        new_proc("cons", procedures::lists::cons),
        new_proc("list", procedures::lists::list),
        new_proc("rest", procedures::lists::rest),
        new_proc("list?", procedures::lists::is_list),
        new_proc("null?", procedures::lists::is_null),
        // works on multiple collection types
        new_proc("nth", procedures::collections::nth),
        new_proc("length", procedures::collections::length),
        // math/numbers
        new_proc("number?", procedures::math::is_number),
        new_proc("float?", procedures::math::is_float),
        new_proc("integer?", procedures::math::is_int),
        new_proc("rational?", procedures::math::is_rational),
        new_proc("=", procedures::math::equals),
        new_proc("<", procedures::math::less_than),
        new_proc(">", procedures::math::greater_than),
        // errors
        new_proc("throw", procedures::errors::throw),
    ]);
    Rc::new(env)
}

fn new_proc(name: &str, func: fn(&[Val]) -> Result<Val, Error>) -> (Rc<Str>, Val) {
    (
        Rc::new(Str::from(name)),
        Val::from(Procedure::new(name, func)),
    )
}

/*
 * The pattern we will use is to have functions be written taking their specific
 * arguments in a stdlib/builtin procs module. The more specific the functions are
 * the easier it is to test them and use them in different contexts. However, this
 * means that all of the procedures we put into the null env need to be created
 * hear in a way that they can be applied to lists and pull out the specific values
 * and deal with any errors and pass the arguments to the actual function. It is
 * possible that it would make more sense to put these next to the functions, but
 * I think keeping them elsewhere makes sense reflecting their use. Perhaps the functions
 * can actually be the procedures and we can do what we did last time to just save
 * the function pointer and the functions can call methods on objects with their
 * arguments. I am not sure, but some need to be setup so that we can test the
 * interpreter with a procedure or two.
 */
