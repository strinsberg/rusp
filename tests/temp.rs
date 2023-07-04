mod helpers;
use helpers::eval_assert;

#[test]
fn temp_to_get_started() {
    eval_assert("5", "5");
    eval_assert(":hello", ":hello");
    eval_assert("\\space", "\\space");
}

#[test]
fn temp_to_check_builtin_procedures_are_added_to_env() {
    eval_assert("(cons 5 #())", "(5)");
    //eval_assert("(throw :fail \"some junk\" 1 2 4 5)", "");
}

#[test]
fn temp_to_check_library_procedures_are_available() {
    eval_assert("(first #(6))", "6");
}
