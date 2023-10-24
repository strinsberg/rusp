mod helpers;
use helpers::eval_assert;

#[test]
fn test_var_construction() {
    eval_assert("(var 5)", "#<var 5>");
    eval_assert("(var [1 2 3 4])", "#<var [1 2 3 4]>");
}

#[test]
fn test_var_deref() {
    eval_assert("(deref (var 5))", "5");
    eval_assert("(deref (var [1 2 3 4]))", "[1 2 3 4]");
    eval_assert("(let [(v (var 5))] @v)", "5");
}

#[test]
fn test_var_set() {
    eval_assert("(let [(v (var 5))] (set! v 10) @v)", "10");
    eval_assert("(let [(v (var 5))] (set! v 10))", "#<var 10>");
}

#[test]
fn test_var_swap() {
    eval_assert("(let [(v (var 5))] (swap! v 10) @v)", "10");
    eval_assert("(let [(v (var 5))] (swap! v 10))", "5");
}

#[test]
fn test_var_update() {
    eval_assert("(let [(v (var 5))] (update! v inc) @v)", "6");
    eval_assert("(let [(v (var 5))] (update! v (lambda [x] 10)))", "5");
}
