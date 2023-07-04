# Rusp

Rusp is a lisp-like language that compiles to rust. This project holds the
rust data structures and runtime elements that make this possible. The companion
compiler is RuspCC and compiled projects use Rusp as a rust dependency when
compiled but rust using cargo.

# Design

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

##
