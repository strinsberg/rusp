# Rusp

Rusp is a lisp/clojure like language. The interpreter is written in rust, though
realistically it could be in any language. The end goal will be to write a
compiler that translates rusp code into rust so that it can be compiled into
binaries that are not interpreted at all. As rusp is a dynamic language there
will have to be some kind of library, if only for the types, to support the
compiled code, but it will not be compiled along with an interpreter to execute
it. Ideally, there will be a reasonable mechanism for importing and calling
proper rust functions within rusp, once the compiler is made, but not in the
interpreter.

The immediate goal is to get some language design stabalized and to write a
working interpreter. Currently, I am perhaps halfway done these two things. I
do have a working interpreter for a subset of the language. However, there are
some issues with language design and with missing or messy elements like macros.
I need to think these things through a bit more and start from the language
I want to have and adjust the interpreter to fit. Currently, the language and
the interpreter keep changing to suit each other and this slowly becomes messy
as they correct back and forth.


# Design

What follows is a breakdown of certain design features, syntax, and semantics.
The two most important parts, in my mind, are how values are bound in the
environment and how functions are created and called. These will be discussed
first and be followed by discussions of the more specific datatypes.

## Binding

Binding is the process of associating values with symbols. All bindings will
be immutable by default. Duplicate bindings within the same scope are an error. 
Nested scopes can create duplicate bindings to shadow those in their parent
scopes. A special value type will be created to allow a bound value to be
updated, but it will require an explicit marking from the programmer.

Upon evaluation of a bound symbol the value associated with it will be returned.
It is an error to evaluate an unbound symbol.


### Def

The simplest bindings are done at the top level of a program with `def`. The
following example binds the symbol `a` to the number 5 in the current namespace
(namespaces will be covered below).
```clojure
(def a 5)
```

### Defn

A function can be created and bound to a symbol using `defn`. The following
example binds a function that takes an argument and returns it (functions will
be covered in more detail below).
```clojure
(defn identity [x] x)
```

### Let

In order to create local bindings that will not be visible to their parent scope
and namespace we can use `let`. A let expression is has a vector of bindings
and zero or more body expressions. A binding, like `def` consists of a symbol
and an expression whose value will be bound to the symbol.
```clojure
(let [sym expr ...] expr ...)
```

In the following example, we bind the result of two additions to `a` and `b`.
Let bindings are bound in order (like let* in scheme). The example would bind
`a` to 3 and `b` to 4. Once the bindings are set then the expressions in the
body of the let are evaluated in order and the result of the final expression is
returned. Here we have only one expression in the body, the symbol `b` so the
result of the let expression will be the value of `b`.
```clojure
(let [a (+ 2 3)
      b (+ a 1)]
  b)
```

If a let expression has an uneven number of forms, a symbol without a value
expression to bind to it, it is an error.

### Var

Since all bindings are immutable by default it is not possible to change or
update their values. For this purpose it is possible to use a `var`. Var is
a like a box for a value that will allow its value to be changed. A var is
bound to a symbol like any other value, but with the `var` function.
```clojure
(def a (var 5))
```
Here we bind a var to the symbol `a`. That var holds the value 5. In order
to access the value in a var we must *dereference* it by prepending the
symbol with `@`. Evaluating `a` will give us the var that holds 5. Evaluating
`@a` will give us the value 5.

We can also change the value in the var in several different ways. The most common
will be to use the `set!` function to provide a new value.
```clojure
(def a (var 5))
;; @a is 5

(set! a 10)
;; @a is 10
```

## Functions

We saw above how to create a function with the `defn` keyword. Functions are
applied to arguments by wrapping the function name and arguments in parenthesis,
as with the `(+ 1 2)` seen in an example above. A function defined with defn
has the following structure:
```clojure
(defn name [args ...] exprs ...)
```
First we have the function name, a symbol that the function will be bound to.
Second we have a collection of symbols to bind the function arguments. Finally,
we have a body of expressions, that like in the `let` form are evaluated in order
and the result of the last one is returned after a function application. Also,
like `let` the argument names are like a local binding of the values passed to
the function as arguments and are available to the expressions in the body
shadowing any bindings with the same name in the parent environment.

The following function takes a single argument and adds 5 to that argument
and returns the result.
```clojure
(defn add5 [x] (+ x 5))
(add5 10) ;; 15
```

Functions can also be passed as values to other functions.
```clojure
(defn add5 [x] (+ x 5))
(defn call-f-on-x [f x] (f x))
(call-f-on-x add5 10) ;; 15
```
This is a contrive example, but shows that the first argument to `call-f-on-x`
is applied as a function to the second argument. We pass `add5` as the function
and 10 as the arguments. However, if we changed the arguments around we would
get an error as `f` would be bound to the number 10 and a number cannot be
applied as a function.


## Datatypes

Below are more detailed descriptions of all of the datatypes contained in rusp.
Most are found in other programming languages and most are similar or identical
to other lisps. However, there are some syntactic and semantic differences.

### Basic Types (atoms)

These are the basic datatypes in rups, called atoms as they are fundamental types
that all other types are built from.
* Symbols - These are collections of characters primarily used for binding values.
          They can be used in other ways, but when evaluated they either return
          a value if they are bound or raise an error if they are not bound.
          Some examples include `hello` `what-is-your-name?` `pizza1234`. many
          characters are valid in symbols, but some have special meanings. most
          special charcters are used as prefixes for the language to identify
          different types of symbols or treat symbols in different ways. for
          example `'` preceding a symbol tells the program not to evaluate it and
          to return the symbol instead of its value.
* Keywords - These are like symbols, but are prefixed with a `:`. A keyword does
          not refer to a value in the environment. When evaluated it will simply
          return itself. Some examples include `:a` `:i-am-a-keyword1234`.
* Numbers - There are 3 ways that numbers are represented in rusp: Integers,
          Floats, and Rationals. An integer is a positive or negative whole
          number: `0`, `1990`, or `-4000`. A float is number with both
          a whole part and a fractional part: `0.0`, `3.14`, `-123.456`. A
          rational number is a fraction that can be represented by two integers
          (maybe not the mathematical definition): `1/2`, `-3/10`, `100/1`. The
          value of rational numbers is that computer limitations can cause
          issues when converting between integers and floats. Whenever possible
          if a computation would change integers to fractional numbers the result
          will be a rational rather than a float. All computation will also deal
          gracefully with different types of numbers, always returning the
          most precise result possible. So if integers are used and no result would
          return a fraction then the result will always remain an integer. If
          integers and rationals are operated on then the result will be rational
          unless it can be converted back to an integer with no loss of precision.
          If any numbers in a computation are floats then the result will remain
          a float. Floats are neve automatically converted back to rationals or
          integers. This can be done by the programmer, but with the knowledge
          that the conversion may not be accurate. It is also possible in
          rust to write binary and hexidecimal literals prefixing numbers with
          `#b` or `#x`. For example, the number 15 could be written as `#b1111`
          or `#xF`.
* Characters - Characters are the textual representation of letters and digits.
          All single charcters are symbols prefixed with a backslash `\a`. Some
          Rusp supports only ASCII characters. This includes all the printable
          charaters, such as `\a`, `\B`, `\9`, or `\!`. It also supports several
          non-printable charcters as named characters, such as `\newline`,
          `\tab`, `\null`,`\slash`, and `\space`.
* Booleans - The boolean values of true and false have special symbols `#t`,
          `#true`, `#f`, `#false`.
* None - Traditionally lisp has `nil` that is used as the absence of a value,
          but also as the empty list and a false value.

## Control Structures

if, cond, do/loop, case/match

## Macros

Use a clojure style macro. It is not quite as user friendly in my opinion compared
to scheme syntax rules, but it will be much easier to implement and will be
easier for me to create macros with lots of flexibility without making the
implementation extremely complex. This will require some syntax elements and
a gensym identifier and env element of some kind.


# TODO

Not all of these have to be done for the above stated goal, but I want to keep
ideas and necessary todos here so I do not lose them.

- [] Finish the design document with all of the syntax and common forms and
     functions. Before working on something in the interpreter make sure that
     the design has been solidified. You can work on it before the design is
     fully finished, but try to finish the design for a feature before working
     on its implementation, if it needs to change from what is existing.
- [] The reader will need to be updated to accomadate some ideas and changes
     in the way things are represented. Most notably I want to update the
     datastructures' literal forms and add one for sets. I would also like to
     make them immutable by default and use something like ! as a prefix to
     designate things mutable. E.g. `#[]` for a vector and `#![]` for a mutable
     vector. I have even considered using a required prefix for macro names
     and using `!` as a prefix for def names to produce vars, or even just
     use `def!` as a way to produce vars, rather than having to be as explicit
     about it when creating them. Though vars would still require `@` to deref
     them.
- [] Rewrite the new structures for things like `let` in code. I think that along
     with this implementation of more difficult things in rust code that I want
     to start passing lists instead of vectors to functions. I think this will
     require some looking at the code to decide if it is appropriate or not. I
     think vectors are probably better for compiled code, but lists make more sense
     in interpreted code as the data will already be in a list, so we can stop
     converting it before passing it to a function. I beleive that since the
     lists are counted we can still easily use the current pattern for how we
     internally use functions with slight changes for the type of the argument
     list. It would also be possible to use the lists value iterator to extract
     the number of args and then match them in a tuple.
     ```rust
     let arg1 = args_iter.next();
     let arg2 = args_iter.next();
     let arg3 = args_iter.next();
     match (arg1, arg2, arg3) {
          (Some(a), Some(b), Some(c)) => {},
          (Some(a), Some(b), None) => {},
          // ...
          _ => Err(),
     }
     ```
     This might not be perfect, and for builtin functions require a little bit
     more to allow taking rest arguments etc. However, it is not much more work
     than the current scheme and a few extra helpers could make it easier.
     For user defined functions being evaluated it might require some adjustment
     to the way those functions work, along with the argument list types, but
     I also want to adjust those to accept things like destructuring, so I will
     have to recongifure them anyway.
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
- [] Functions that can compile the internal structure to the rust structure
     necessary to create the structure in rust without reading.
- [] A scheme of compiling the basic structures straight to rust. Something that
     is more rustlike than we had in the previous tests. Using var and a nicer
     way to design a closure (tdb) should make it a lot better. I really do
     want to scope it and not need an environment at runtime, which is mostly
     necessary for closures, but if the bindings that are mutable are vars then
     there is nothing but cloneing to do when capturing an environment, which
     means that we do not have to do anything funny for creating closures.


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
