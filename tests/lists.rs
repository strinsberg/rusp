mod helpers;
use helpers::eval_assert;

#[test]
fn test_list_construction() {
    eval_assert("(list 1 2 3 4)", "(1 2 3 4)");
    eval_assert("#(1 2 3 4)", "(1 2 3 4)");
    eval_assert("(cons 1 #())", "(1)");
    eval_assert("(cons 1 #(2 3 4))", "(1 2 3 4)");
    eval_assert("(cons 1 2)", "(1 2)");
}

#[test]
fn test_list_predicates() {
    eval_assert("(list? #())", "#t");
    eval_assert("(list? #(1 2 3 4))", "#t");
    eval_assert("(null? #())", "#t");
    eval_assert("(null? #(1 2 3 4))", "#f");
}

#[test]
fn test_simple_list_access() {
    // elementwise
    eval_assert("(first #(1 2 3 4 5))", "1");
    eval_assert("(second #(1 2 3 4 5))", "2");
    eval_assert("(third #(1 2 3 4 5))", "3");
    eval_assert("(fourth #(1 2 3 4 5))", "4");
    eval_assert("(fifth #(1 2 3 4 5))", "5");
    // by index
    eval_assert("(nth 0 #(1 2 3 4 5))", "1");
    eval_assert("(nth 1 #(1 2 3 4 5))", "2");
    eval_assert("(nth 4 #(1 2 3 4 5))", "5");
    // rest
    eval_assert("(rest #(1))", "#()");
    eval_assert("(rest #(1 2 3 4 5))", "(2 3 4 5)");
    eval_assert("(rest (rest #(1 2 3 4 5)))", "(3 4 5)");
    // missing
    eval_assert("(first #())", "#none");
    eval_assert("(nth 3 #(1))", "#none");
    eval_assert("(rest #())", "#()"); // should be empty or #none?
}

#[test]
fn test_information() {
    eval_assert("(length #())", "0");
    eval_assert("(length #(1 2 3 4))", "4");
    eval_assert("(empty? #())", "#t");
    eval_assert("(empty? #(1 2 3 4))", "#f");
}
