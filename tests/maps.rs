mod helpers;
use helpers::eval_assert;

#[test]
fn test_map_construction() {
    // Note as map is unordered it is difficult to test complex creation
    // without other map functions, so we will keep it simple here and
    // add more below after other functions are tested.
    eval_assert("(table :a 1)", "{:a 1}");
    eval_assert("{:a (+ 1 2)}", "{:a 3}");
    eval_assert("(table)", "{}");
    eval_assert("{}", "{}");
    eval_assert("(dict :a 1)", "#{:a 1}");
    eval_assert("#{:c (+ 1 2)}", "#{:c 3}");
    eval_assert("(dict)", "#{}");
    eval_assert("#{}", "#{}");
}

#[test]
fn test_map_predicates() {
    eval_assert("(table? {})", "#t");
    eval_assert("(table? {1 2 3 4})", "#t");
    eval_assert("(dict? {1 2 3 4})", "#f");
    eval_assert("(table? #{1 2 3 4})", "#t");
    eval_assert("(dict? #{1 2 3 4})", "#t");
    eval_assert("(table? #(1 2 3 4))", "#f");
    eval_assert("(null? {})", "#f");
    eval_assert("(empty? {})", "#t");
    eval_assert("(empty? {1 2 3 4})", "#f");
}

#[test]
fn test_map_access() {
    eval_assert("(get {:a 4 :b 6} :a)", "4");
    eval_assert("(get #{:a 4 :b 6} :a)", "4");
}

#[test]
fn test_map_mutation() {
    eval_assert(
        "(let [(m (assoc! {:a 4 :b 6} :c 8 :d 10))]
           [(get m :a) (get m :b) (get m :c) (get m :d)])",
        "[4 6 8 10]",
    );
    eval_assert("(dissoc! {:a 4 :b 6 :c 8} :a :b)", "{:c 8}");
    eval_assert("(clear! {:a 4 :b 6 :c 8})", "{}");
    eval_assert(
        "(let [(m {:a 4 :b 6})
               (m2 {:a 9 :c 8})
               (m3 {:b 11 :d 10})]
           (merge! m m2 m3)
           [(get m :a) (get m :b) (get m :c) (get m :d) (get m2 :d) (get m3 :a)])",
        "[9 11 8 10 #none #none]",
    );
    // non-destructive merge will be concat
}
