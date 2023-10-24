mod helpers;
use helpers::eval_assert;

#[test]
fn test_vector_construction() {
    // TODO new-vector (or something like that)
    eval_assert("(vector 1 2 3 4)", "[1 2 3 4]");
    eval_assert("(tuple 1 2 3 4)", "#[1 2 3 4]");
    eval_assert("[1 2 3 4]", "[1 2 3 4]");
    eval_assert("#[1 2 3 4]", "#[1 2 3 4]");
    eval_assert("[1 2 (= 1 2)]", "[1 2 #f]");
    eval_assert("#[1 2 (= 1 2)]", "#[1 2 #f]");
    eval_assert("(push! [1 2 3] 4)", "[1 2 3 4]");
    eval_assert("(pop! [1 2 3 4])", "[1 2 3]");
}

#[test]
fn test_vector_predicates() {
    eval_assert("(vector? [])", "#t");
    eval_assert("(vector? [1 2 3 4])", "#t");
    eval_assert("(tuple? [1 2 3 4])", "#f");
    eval_assert("(vector? #[1 2 3 4])", "#t");
    eval_assert("(tuple? #[1 2 3 4])", "#t");
    eval_assert("(vector? #(1 2 3 4))", "#f");
    eval_assert("(null? [])", "#f");
    eval_assert("(empty? [])", "#t");
    eval_assert("(empty? [1 2 3 4])", "#f");
}

#[test]
fn test_vector_access() {
    // elementwise
    eval_assert("(first [1 2 3 4 5])", "1");
    eval_assert("(second [1 2 3 4 5])", "2");
    eval_assert("(third [1 2 3 4 5])", "3");
    eval_assert("(fourth [1 2 3 4 5])", "4");
    eval_assert("(fifth [1 2 3 4 5])", "5");
    // by index
    eval_assert("(nth 0 [1 2 3 4 5])", "1");
    eval_assert("(nth 1 [1 2 3 4 5])", "2");
    eval_assert("(nth 4 [1 2 3 4 5])", "5");
    // missing
    eval_assert("(first [])", "#none");
    eval_assert("(nth 3 [1])", "#none");
}

#[test]
fn test_tuple_access() {
    // elementwise
    eval_assert("(first #[1 2 3 4 5])", "1");
    eval_assert("(second #[1 2 3 4 5])", "2");
    eval_assert("(third #[1 2 3 4 5])", "3");
    eval_assert("(fourth #[1 2 3 4 5])", "4");
    eval_assert("(fifth #[1 2 3 4 5])", "5");
    // by index
    eval_assert("(nth 0 #[1 2 3 4 5])", "1");
    eval_assert("(nth 1 #[1 2 3 4 5])", "2");
    eval_assert("(nth 4 #[1 2 3 4 5])", "5");
    // missing
    eval_assert("(first #[])", "#none");
    eval_assert("(nth 3 #[1])", "#none");
}

#[test]
fn test_vector_mutation() {
    eval_assert(
        "(let [(v [1 2 3 4])]
                   (push! v 5)
                   (= (length v) 5))",
        "#t",
    );
    eval_assert(
        "(let [(v [1 2 3 4])]
                   (pop! v)
                   (= (length v) 3))",
        "#t",
    );
    // TODO Uncomment when we have better errors
    // eval_assert("(push! #[1 2 3] 4)", "error");
    // eval_assert("(pop! #[1 2 3])", "error");

    // TODO set-nth!, vector-fill!
}

#[test]
fn test_vector_info() {
    eval_assert("(length [])", "0");
    eval_assert("(length [1 2 3 4])", "4");
    eval_assert("(length #[])", "0");
    eval_assert("(length #[1 2 3 4])", "4");
}

#[test]
fn test_vector_conversion() {
    eval_assert("(vector->tuple [1 2 3 4])", "#[1 2 3 4]");
    eval_assert("(tuple->vector #[1 2 3 4])", "[1 2 3 4]");
    eval_assert(
        "(let* [(v [1 2 3 4])
                        (t (vector->tuple v))]
                    (push! v 5)
                    #(v t))",
        "([1 2 3 4 5] #[1 2 3 4])",
    );
    eval_assert(
        "(let* [(t #[1 2 3 4])
                        (v (tuple->vector t))]
                    (push! v 5)
                    #(v t))",
        "([1 2 3 4 5] #[1 2 3 4])",
    );
    eval_assert(
        "(let [(v [1 2 3 4])]
                    (vector-freeze! v)
                    (tuple? v))",
        "#t",
    );
    // TODO vector->list
}

// Had some issues above returning [v t] as a vector literal. It returned
// [v t] instead of [[1 2 3 4 5] #[1 2 3 4]]. These are to test to make sure
// that in this context, and perhaps others, that literal vectors are being
// evaluated properly.
#[test]
fn test_collection_lit_with_symbols_in_let() {
    eval_assert("(let [(a 4) (b 6)] #(a b))", "(4 6)");
    eval_assert("(let* [(a 4) (b a)] a b #(a b))", "(4 4)");
    eval_assert("(let [(a 4) (b 6)] [a b])", "[4 6]");
    eval_assert("(let* [(a 4) (b a)] a b [a b])", "[4 4]");
    eval_assert("(let [(a 4) (b 6)] #[a b])", "#[4 6]");
    eval_assert("(let* [(a 4) (b a)] a b #[a b])", "#[4 4]");
}
