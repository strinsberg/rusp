# Notes

General notes, todos, etc. for the project and plans for other related projects.

# New Ideas

## Compiler
The compiler is going to be written in clojure, with the hope that it can use a subset that is valid rusp code too. The ultimate goal is to transform the rusp code into something close to an assembly like code. Idealy some level of this will be easy to transform to rust. Perhaps there will be a middle level that can diverge to rust or to other targets. Otherwise for now I can just target it directly to rust, but it could be transformed to a really low level and then transformed from there back to rust.

The result could look something like this
```clojure
(ns something)

(def a 8)

(cons 1.5 2/3)

(let [a "hello"
      b "world"]
  (println (str a " " b)))
```

goes to something like

```clojure
(ns something)

(val-assign a (val-int 8))

(cons (val-float 1.5) (val-rational 2 3))

(block-assign temp0)
(val-assign a (val-string "hello"))
(val-assign b (val-string "world"))
(val-assign temp1 (val-string " "))
(val-assign temp2 (vec! a temp1 b))
(val-assign temp3 (apply str temp3))
(tail-call temp3)
(block-end)

(end something)
```

There are a few things to note here:
* A def or let variable assignment is just a rust let with whatever the val type is and a constructor for the value.
* A let is just a nested block.
* A block and namespace could be inside a list or as showed here with something to delimit the start and end
* A builtin simple function call can just be the call, but a defined function will use apply.
* We can simplify function calls a little by creating temp vars to hold the results of any expressions in the argument list. I am not sure if when compiled rust will optimize this and remove the intermediate variables. The rust code would just move those rather than clone them, so maybe. The value in this representation is that it gets closer to 3 address code and might be easier to optimize or to convert directly to things like LLVM if desired.
* The block assign is to easily store the result of the computation inside the block in a variable outside the block. For a top level let like this I guess it is not actually necessary.
* The tail-call is something that I can add to every tail expression, or only to those expressions that might have returned a tail-call object. Essentially, the tail part of every expression will be lazy if it is a function call. After it returns the result needs to be processed until there is no more tail calls. So, a recursive function would have a tail call that would be returned and then executed in a loop until the final value is obtained. This will eliminate stack use. However, given that we are adding a function call to every tail expression this will add some overhead. It is possible that we could use recur like clojure to indicate when to do this to the compiler, or do some analysis and identify which function we are in and only wrap things in tail calls when we return from this exact function. Using recur as a keyword might be good to indicate to the user that we are adding a tail call, but we would still need to store the name of the function to use in the tail call, so maybe it is not that helpful, unless we just transform the body into a loop with it and do not return the function at all. The only difficulty here is ensuring we are in tail position when transforming. I think for now just using tail-call everywhere is the easiest and we can optimize them away, but to do that might need to store some meta data in the calls. Obviously. since the let above is a top-level expression and we are not even returning the value for anything it does not need a tail call.

## Organization
The overall organization may change some if I write the compiler in clojure. The standard lib and rusp data structures will need to be setup with rust project structure if we want them to be compiled and testable. Ideally, the rust data will be in a single file and make it easy for us to either collect through an include/import in a cargo deps file or setup to be textually inserted as a module in the resulting compiled file. Probably both would be good. The compiler really should make it possible to compile a .rs file for use as a binary, i.e. rustc can just be called and compile it straight to a binary. But for larger projects, or me building standard libs in rusp and then compiling them to files that can be included or copied into those files. A rusp project should not require a single file and should after compiling be capable of being compiled with cargo to pull together a bunch of rusp libs.

All of that to say that my project needs to separate the concerns a bit, and maybe to be organized as a rust project so that it could be included as a cargo dependency when desired, perhaps even for the interpreter, that could be split into a separate project, that might itself be compiled? The difficulty is that if the compiler is supposed to be a binary, hard since it is built with clojure sadly, unless we can compile it with grall, that it also needs to access the source code. I do not really know how to reconcile this inconsistency. I suppose that eventually the compiler can be compiled using the compiler and then it will no longer require clojure, only a copy of the rusp data file and the rusp stdlib which could perhaps be downloaded individually from git and used where needed.

So, for now what we need to do is to have the rust project built as it would be for the lib. Then it should have a separate folder for the clojure source code that will run the compiler. We might have a separate folder for tests or not idk.


# OLD
As of March 17

# Testing

I am not happy doing the integration type testing in rust. It is very easy to
do, but it is somewhat brittle in that if a single test fails then everything
stops. What I really want to be able to do is write a full test suite for all of
the features of rust, from scanning to core procedures, and just run it and be
able to see what tests pass and which do not. It could easily have the ability
to only run a specific test file/suite when working on specific funcitonality.
It is also not that hard to run each test suite in parallel and display the
results when all test suites are done.

I can just do this with a single clojure file as a test lib and call it with a
bb task. The test lib should be generic enough that tests can be setup and be
run for both the interpreter and compiler with the same test suite. I am
thinking that declaring the tests in edn and then having a wrapper for
a test runner, or have the test runner take a function, would be easiest. Then
any program can either setup its own runner or pass its own evaluation function
to the runner to be called with all the data. This would be a cli testing program
similar to the one I made with python and can just pass a single string to stdin
and expect a single string back in stdout, or stderr as well. Perhaps later a
second test type could be added to do something more like expect and take a
series of tuples with the send and recieve strings, or even regex.

For now what this means for rusp is that I want to create a `bb.edn` and a
`bb-cli-test.clj` in the rusp project. These can be used to automate things that
I am not doing with cargo, and perhaps have wrappers for cargo if desired.

Once the lib is complete I will want to start writting the tests. They will serve
as a full spec for the language and its syntax. This can both drive the refinement
of the syntax and builtin structures and procedures as well as ensure that as
each feature is added it has tests that will confirm it is working properly. It
can start simple to ensure that language refinement does not break large numbers
of tests. E.g. do not get carried away with full complex testing until the general
syntax and procedures have solidified.


# Compiler

The compiler will be written in rusp along with a core library. Because of the
nature of the compilation many basic forms will be compiled and will not need
to call any rust functions. E.g. like the procedurues placed in the env in
the interpreter. The core library will then also be written in rusp and compiled
to a single file `rusp-core.rs`.

At the beginning the compiler can be run with
the interpreter, but it will eventually be compiled. Once compiled to rust it
will require rust libs for core and data to be available to run. This is also
true of any but the most basic compiled programs. So, early on it may be
necessary to make the core lib in compiled rusp/rust by hand. The data lib should
be available from the interpreter, and eventually broken out into its own lib.
All we should need from the current interpreter is the data structures. Actually,
this is where there may need to be some kind of handwritten rust procedures, or
the data structures themselves may need methods that follow the calling convention
of the compiled code. With the second option then core datastructure code could
be dealt with by the compiler rather than a lib. However, it might be cleaner
to have a basic wrapper lib for the data that follows the rust conventions for
procedures to allow compiling more easily.

In the above case we would need three
libraries for compile code to run. `rusp-core.rs` which will now be the basic
required procedures and could be kept in the same package as the data lib.
`rusp-data.rs` will contain all the data definitions, classes, traits, errors,
etc. that will be used in compiled rusp code. Will be used by all other libs
and programs that work with rusp. `rusp-std.rs` will be the be the extended
core library containing all of the rusp builtin procedures and data structures
that can be implemented in rusp and compiled.

The compiler compilation process will be:
- Use the interpreter to run the compiler code on the compiler code
- Use the interpreter to run the compiler code on the stdlib
- If necessary pull the latest core and data libs
- Use the rust compiler (or cargo) to compile the final executable for the
  rusp compiler with core, data, and std lib dependencies

Other compiled files can be compiled using the resulting compiler. It should
have a few command line arguments for various things:
- Init - To create a new rusp project designed for compilation. This will probably
         be for creating a cargo toml so that cargo can be used to build and
         manage dependencies, once the libs all have proper github repos.
- Build - To generate the rust files for any rusp source files and to call
          cargo build.
- Compile - To generate the rust files for any rusp source files without building.
- Release - To generate the rust files and compile with cargo release.
