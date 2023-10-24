mod helpers;
use helpers::eval_assert;

#[test]
fn test_number_predicates() {
    // number?
    eval_assert("(number? 100)", "#t");
    eval_assert("(number? 1.09)", "#t");
    eval_assert("(number? 1/9)", "#t");
    // float?
    eval_assert("(float? 100)", "#f");
    eval_assert("(float? 1.09)", "#t");
    eval_assert("(float? 1/9)", "#f");
    // integer?
    eval_assert("(integer? 100)", "#t");
    eval_assert("(integer? 1.09)", "#f");
    eval_assert("(integer? 1/9)", "#f");
    // rational?
    eval_assert("(rational? 100)", "#f");
    eval_assert("(rational? 1.09)", "#f");
    eval_assert("(rational? 1/9)", "#t");
    // exact?
    eval_assert("(exact? 100)", "#t");
    eval_assert("(exact? 1.09)", "#f");
    eval_assert("(exact? 1/9)", "#t");
}

#[test]
fn test_number_comparisson_predicates() {
    // zero?
    eval_assert("(zero? 100)", "#f");
    eval_assert("(zero? -100)", "#f");
    eval_assert("(zero? 0)", "#t");
    eval_assert("(zero? 0.0)", "#t");
    eval_assert("(zero? 0/9)", "#t");
    // positive?
    eval_assert("(positive? 100)", "#t");
    eval_assert("(positive? -100)", "#f");
    eval_assert("(positive? 0)", "#f");
    // non-negative?
    eval_assert("(non-negative? 1.00)", "#t");
    eval_assert("(non-negative? -1.00)", "#f");
    eval_assert("(non-negative? 0)", "#t");
    // negative?
    eval_assert("(negative? 1/8)", "#f");
    eval_assert("(negative? -1/8)", "#t");
    eval_assert("(negative? 0)", "#f");
}

#[test]
fn test_arithmetic() {
    // Does not need to be a full test of each possible combination, as this
    // is done in number.rs. However, it cannot hurt to run tests for a function
    // here to ensure that common uses and possible edge cases are respected.
    // add/sum
    eval_assert("(+)", "0");
    eval_assert("(+ 5)", "5");
    eval_assert("(+ 1 2 3 4 5)", "15");
    eval_assert("(+ 1 2 3.1 4 5)", "15.1");
    eval_assert("(+ 1/2 1/4)", "3/4");
}
