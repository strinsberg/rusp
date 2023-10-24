# Rusp

Rusp is a lisp-like language that compiles to rust. This project holds the
rust data structures and runtime elements that make this possible. The companion
compiler is RuspCC and compiled projects use Rusp as a rust dependency when
compiled but rust using cargo.

The goal for now is to get a reasonable subset of functions and types finished.
Something close to the r5rs scheme and some very common clojure functions to
cover the differences in the languages. It is important to come up with at least
a pretty solid idea of what the semantics of the language are. This should also
have at least a document here if not a web page attached to the project on my
site. Finally, setting this up like I did the scheme interpreter with a web
assembly project. The basic need would be an interpret function that I could
call on text and return the string of the result. A slightly better implementation
would be one that could support creating an interpreter and calling eval on
it to do the same as above, but with an additional bonus of maintaining the env
and being useable for a repl or some kind of notebooklike page.


# TODO

Not all of these have to be done for the above stated goal, but I want to keep
ideas and necessary todos here so I do not lose them.

- [] Rewrite macro classes. They do not treat all cases properly. They do not
     have to be as full as scheme macros, but we want to be able to do a fair
     amount with them. I need more thought into how to rename bindings, or if
     I will bother. How to deal with symbols that are not defined in the macro,
     do they just get put in as literals or do they get linked/replaced with
     there value in the current environment where the macro is created. I also
     need to deal with captures better so that, (var val) ... is possible to
     expand with the elipses in a template. I currently capture all instances
     of var and val when I match, but they are not grouped with the list. If the
     template contains literals etc. then it cannot be expanded properly as we
     have no way of knowing the number of captures. At the very least we need
     to keep a list for each captured var rather than naming them in a map with
     indicies. Then we can take a list like this and expand it and use one
     of the vars to decide how big it is and substitute them in the template list
     each time based on index. We can also replace literals in each one. Some
     care needs to be taken to make sure that we don't just implement a rule for
     this exact situation, but that longer lists and vectors etc. could also be
     expanded with the elipse.
- [] More builtin macros. There are a number of derived expressions that we could
     still use to make programming better and allow for making even more lib
     functions without having to program them in rust.
     - [] Defn
     - [] Do/loop
     - [] Cond
- [] More library functions period.
     - [] List functions, though there will be less of them since they are
          immutable.
     - [] Map functions
     - [] String functions
     - [] More math and arithmetic functions
- [] We need an error type and the associated functions. I have a throw function
     but not a proper error type and associated functions. We need to both be
     able to capture the errors that are made in rust and set them up nicely in
     the type that is the rusp error. It would be nice to have types as keywords
     or even provide a symbol that becomes the error type and is an actual type,
     though we do not have types of other kinds. Maybe the error could just
     be a map with some manditory feilds or something. Or maybe it could be a
     type and have some accessor methods, or just have an underlying map type
     so that it could be accessed with keywords like maps in clojure.
- [] Either a type system or a way to do things like multimethods in clojure.
     could do something exactly like multimethods or enable some kind of type
     creation that a get-type method would return for. Using maps is great way
     of doing types, but it would be nice if there was a way to create a type
     that could be checked more easily. Perhaps even just a way to promote a
     map to a type. I don't like prototype languages. I do like structs with a
     concrete type and some way to access methods and fields. I also like traits
     and other things. Since the langauge is not type checked it is all up to the
     user to type check things in their functions. Closures can be used with vars
     to implement types. I guess the biggest question is not how they are implemented
     underneath, but how you create and interact with them. Things like
     multimethods are a realy nice way to introduce algorithms that work on
     disparate types. Pattern matching can also be a way to more easily switch
     on types and more. Since we have maps multimethods are a really simple
     way to provide some kind of polymorphic behaviour, which imo is super
     useful. But some kind of trait and implement system or enum system like
     rust has could be super nice too, though I would want to make sure my builtin
     types worked with it to.


# Design

**TODO** this section is no longer correct. See the reader for more info.

## Data Types

* Symbol - Evaluate to other data that has been bound unless quoted. Ex. `hello 'hello`.
* Keyword - Evaluate to themselves. Ex. `:hello`.
* Boolean - True and false. All values are considered true in conditions except for false. Ex. `#t #true #f #false`.
* Number - 64 bit integers, floats, and rationals (using 2 64 bit integers). Ex. `1234 12.34 12/34`.
* Character - Ascii characters only for uppercase, lowercase, digits, and symbols. Special characters are only available for space, tab, newline, and null. Ex. `#\a #\B #\* #\space #\tab #\newline #\null`.
* String - Collections of characters inside double quotes. Only accept escape the following escape sequences `\\ \t \n \0 \"`. Ex. `"Hello there \t What is your name? \n"` or `"This is a slash \\ in a string`.
* List - Immutable shared suffix linked lists built with `cons` similar to other lisps. There are no dotted plists like in scheme, a list always ends with the empty list. List literals are written with a quote or the `list` functions. Ex. `'() '(1 2 3 4) (list 1 2 (+ 1 2) 4)` and `(cons 1 2) -> '(1 2)`.
* Vector - Mutable growable arrays similar to other programming languages. Created with literal syntax using `[]`. Ex. `[1 2 3 4]`.
* Tuple - Immutable arrays. It is an error to call vector mutation procedures such as `push!` and `pop!` on a tuple. Tuples can be copied into an immutable vector with `tup->vec`. Tuples use vector literal syntax prefixed with a hash. Ex. `#[1 2 3 4]`.
* Hash Map - Mutable hash maps. Only accept symbols, keywords, strings, numbers, characters, and booleans as keys. Values can be any type. Hash maps can have keys added, updated, or removed. They are created with literal syntax using `{}`. Ex. `{:a 10 :b 90 #\c '(1 2 3 4)}`.
* Dictionary - Immutable hash maps. Function the same as Hash Maps but it is an error to call mutating procedures on a dictionary. Can be copied to a hash map with `dict->map`. Like tuples they are created with the same syntax as hash maps prefixed with a hash. Ex. `#{:a 3 :b 9 #\c '(1 2 3 4)}`.
* Procedures - Builtin library functions. Ex. `(cons 1 2)` or `cons -> #<procedure cons>`.
* Closures - Anonymous functions that capture their environment. These are defined with `lambda` with scheme-like syntax. Ex. `(lambda (a b) (+ a b)) -> #<closure>`. Closures can also be defined with `defn` at the top level and will keep track of their name. Ex. `(defn f (a b) (+ a b)) -> #<closure f>`.
* Error - An error type with a collection of error information. There are several builtin error types that can be created with function calls. Ex. `(type-error expr value expected-type)`. There is also one generic error type that takes a keyword for type, a message or closure to produce a message from the arguments, and optional data elements. Ex. `(error :my-error "you made a mistake" val expr ...)` or `(error :my-error (lambda (args) (str "you made a mistake: " (first args) " instead of " (second args))) arg1 arg2)`. There should also be a `panic!` since errors are values. Panics just take a message and end the program.
* None - The absence of a value. This is similar, but not the same as, nil in lisps. None is not the empty list and it is not a false value. It is used more as would be Option in rust witout an explicit Some variant. Ex. `(if (is-there? val) val none)`. Can be checked for with `none?` or `if-some` to bind the result only when it is not none (like `if-let` but explicit since `none` is not a false value).


## Binding

Top level definitions use `def` and `defn`. Local bindings use `let` or `letrec`,
where both function like `let*`.

```scheme
(def a 5) ;; a is now 5
(defn inc (x) (+ x a)) ;; function to add 5 to its argument (errors if x is not number?)
(let ((a 9) (b 10)) (+ a b)) ;; result is 19 as a in the let shadows the top-level a
```
