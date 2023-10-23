mod helpers;
use helpers::eval_assert;

#[test]
fn test_let_macro() {
    eval_assert("let", "#<macro let>");
    eval_assert("(let [(a 5)] a)", "5");
    eval_assert("(let [(a 5) (b 6)] 1 2 (= a b))", "#f");
    eval_assert("(let [(a 5)] (let [(a 99) (b a)] b))", "5");
}

#[test]
fn test_let_star_macro() {
    eval_assert("let*", "#<macro let*>");
    eval_assert("(let* [(a 5)] a)", "5");
    eval_assert("(let [(a 5)] (let* [(a 99) (b a)] b))", "99");
}

#[test]
fn test_and_macro() {
    eval_assert("and", "#<macro and>");
    eval_assert("(and)", "#t");
    eval_assert("(and #t)", "#t");
    eval_assert("(and #f)", "#f");
    eval_assert("(and (= 1 1) #f)", "#f");
    eval_assert("(and #f (= 1 1))", "#f");
    eval_assert("(and #t 10 #t)", "#t");
    eval_assert("(and #t #t #())", "#()");
    eval_assert("(and #t #none #t)", "#f");
    eval_assert("(and #t #t #none)", "#none");
    eval_assert("(and #t #t #f)", "#f");
}

#[test]
fn test_or_macro() {
    eval_assert("or", "#<macro or>");
    eval_assert("(or)", "#f");
    eval_assert("(or #t)", "#t");
    eval_assert("(or #f)", "#f");
    eval_assert("(or (= 1 1) #f)", "#t");
    eval_assert("(or #f (= 1 1))", "#t");
    eval_assert("(or #f 10 #f)", "10");
    eval_assert("(or #f #f #())", "#()");
    eval_assert("(or #f #none #f)", "#f");
    eval_assert("(or #f #f #f)", "#f");
}
