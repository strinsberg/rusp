use crate::data::*;

// Difficulties
//
// A rust module cannot have variables that are initialized at runtime
// easily. I.e. having def become a let at the top level is not possible.
// So, a rups module has to be creative. However, I would still like to
// try to get functions to load just like any other rust module, so maybe
// wrapping the others in some kind of struct that can be loaded like the
// module in runtime and imported and accessed similar to any other rust code.
// The real question becomes how the code will be used. Do I intend that one
// could easily call rusp from rust and rust from rusp? or one of the two?
// or neither?

// Based on the previous runtime attempt and some thought I think like clojure (i think)
// the easiest thing to do is to compile an ns/module to a class that exposes
// the contents of the file. This would allow running the code just like
// in a lisp file. The trick is to ensure that arbitrairy code is only executed
// once and to ensure that rust variables are moved/copied/cloned properly
// everywhere they are used. Inside a module all top level functions can be
// methods, though to pass them as vals they will have to be wrapped in an
// appropriate structure or rust lambda, which is annoying, unless they have
// the exact same signature as lambdas. In which case it may be easiest to
// just make them a Val from the start. Maybe the way to do captures is to
// create a structure with a simple map of captured vars where they can be
// looked up. A mini env if you will, though the compiler must know which elements
// are expected to be in that env and which are not. Also, what counts as
// captured and what counts as just used from the surrounding env?

// Have to wrap arbitrairy statements in a function
fn wrapper() {
    // (def a 5)
    let a = Val::from(5);

    // (def g (lambda [x y & z] a))
    let g = Val::lambda(
        // Env::new(&[("a", a.clone())]),
        |args: &[Val], captures: &Env| {},
    );

    // (def b (g 1 2 3 4))
    let b = g.apply(&vec![1 2 3 4]);
}

// (defn f [x y & z] 5)
pub fn f(x: Val, y: Val, z: &[Val]) -> Val {
    Val::from(5)
}

// The compiler needs to keep an env that tracks the types of things at the
// very least. This is important for function/lambda application as a function
// will be applied directly as a method in the mod and a lambda will not be.
// Of course this might make for some awkwardness when calling functions in
// other modules. Maybe it is just easier to make all functions lambdas and
// call them on a vector of arguments. It again comes down to what the goal
// of the compilation is, to compile to rust so it can be run as a rust program
// or imported as a rusp module etc.? Or if we want to compile directly to rust
// as a way of writing rust differently or fully interoping with rust? I feel like
// a big part of me wants to be able to compile a lisp directly to rust, but
// I recognize that there are very large differences in how the two languages
// are evaluated. Lisp is dynamic and very anything goes. Rusp is very strict
// and static and only a certain subset of things is ok. I could probably
// make a typed lisp that followed some of rusts rules, but would it be any
// better than writing rust itself? Plus the compile errors in the latter would
// be super unfortunate as the rewritten code would be hard to diagnose. Plus
// do we want to be writing clone and & and * everywhere? I could possibly
// simplify some things so that it could be rust just with some helpers, and
// maybe even an interpreter. But would it be worth it at all? I could not
// use rusts tools on the lisp like code. Is the main feature a scripting
// language that is rust, but dynamic? Because I could do something like that
// With the values I already have, but it is obviously much more complicated
// to compile and have useable along with rust. How would one call the scripted
// code from rust and how would one call rust code from the scripted code.
// Along those lines the idea of an embeded scripting language in rust is another
// way this could be used, but that also has different requirements. How do
// you pass rust structures into the dynamic language at runtime and get them
// out again? How do languages like lua deal with that? Obviously lua code
// interoping with C code is not an easy one to one transformation. And is this
// at all what my goal might be? Embeding a dsl is a whole other thing imo. It
// involves having an iterpreter and an API for communicating between the two
// langauges. In a way I might have these things, but it is complicated. Plus
// I do not really want to embed an interpreter in rust programs to write
// interpreted code along with my rust. I want to compile to rust in some way.
// I think that the most useful way of doing this is just to have a language that
// compiles to rust, that may not be the fastest rust, but reflects the new
// language as well as possible. Then the new language can call rust code or
// libraries to use rust code. Then the new language is used as the primary
// way to write a project, but it can use existing high quality rust code
// and if desired one can write code that needs to be high performance directly
// in rust and perform the slow conversion between values on the way in and out
// but only once and speed up the algorithm by writing it in the best rust possible.
// And for many uses probably the compiled dynamic code will be more than enough.
