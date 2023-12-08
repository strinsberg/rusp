# Notes

General notes, todos, etc. for the project and plans for other related projects.

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